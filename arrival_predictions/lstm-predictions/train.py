import torch
import torch.nn as nn
import torch.utils.data as tud
import pytorch_lightning as pl
import pytorch_lightning.callbacks as plc
import pytorch_lightning.loggers as pll
from hsml.schema import Schema
from hsml.model_schema import ModelSchema
import joblib
import os

from model import *
from dataset import *
import hopsworks

# connect to feature store
project = hopsworks.login(project="zeihers_mart")
fs = project.get_feature_store()

torch.multiprocessing.set_sharing_strategy("file_system")
torch.manual_seed(0)

# load dataset
dataset = MetroDelayDataset(data_file="./data/training_data.txt", vocab_file="./data/training_data_vocab.json", feature_store=fs)
dataset_train, dataset_val, dataset_test = tud.random_split(dataset, [0.7, 0.15, 0.15])
print("Vocab size:", dataset.vocab_size)
print("Training samples:", len(dataset_train))
print("Validation samples:", len(dataset_val))
print("Test samples:", len(dataset_test))

batch_size = 64
# think about shuffling or not
train_loader = tud.DataLoader(dataset_train, batch_size=batch_size, num_workers=0)
val_loader = tud.DataLoader(dataset_val, batch_size=batch_size, num_workers=0)
test_loader = tud.DataLoader(dataset_test, batch_size=batch_size, num_workers=0)

# load model
model = MetroPredictionLSTM(
    vocab_size=dataset.vocab_size,
    hidden_size=64,
    num_layers=1,
    dropout=0.1,
    pad_index=dataset.pad_idx,
)
"""
model = MetroPredictionLSTM.load_from_checkpoint(
    "tb_logs/metro-delay-logger/version_21/checkpoints/epoch=66-step=871.ckpt"
)
"""

# logger
logger = pll.TensorBoardLogger("tb_logs", name="metro-delay-logger")

# training
trainer = pl.Trainer(
    accelerator="auto",
    min_epochs=10,
    max_epochs=200,
    logger=logger,
    gradient_clip_val=None,
    callbacks=[
        # Early stopping
        plc.EarlyStopping(
            monitor="val_loss",
            mode="min",
            patience=3,
            verbose=True,
            strict=True,
            min_delta=1e-4,
        ),
        # saves top-K checkpoints based on "val_loss" metric
        plc.ModelCheckpoint(
            save_top_k=1,
            monitor="val_loss",
            mode="min",
        ),
    ],
)

trainer.fit(
    model,
    train_dataloaders=train_loader,
    val_dataloaders=val_loader,
)

metrics = trainer.test(model, test_loader)

mr = project.get_model_registry()

input_schema = Schema(dataset.data[0])
output_schema = Schema(dataset.data[0])
model_schema = ModelSchema(input_schema=input_schema, output_schema=output_schema)
model_schema.to_dict()

model_dir = "torch_tf_model"
if os.path.isdir(model_dir) == False:
    os.mkdir(model_dir)

# Save the model
joblib.dump(model, model_dir + '/metro_delay_lstm.pkl')

# Create a model in the model registry
model = mr.torch.create_model(
    name="metro_delay_model",
    description="PyTorch Lighting LSTM Model predicting the delay at the next stop.",
    model_schema=model_schema,
    metrics=metrics[0]
)

model.save(model_dir)
