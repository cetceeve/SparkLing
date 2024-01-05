import torch
import torch.nn as nn
import torch.utils.data as tud
import pytorch_lightning as pl
import pytorch_lightning.callbacks as plc
import pytorch_lightning.loggers as pll

from model import *
from dataset import *

torch.multiprocessing.set_sharing_strategy("file_system")
torch.manual_seed(0)

# load dataset
dataset = MetroDelayDataset(data_file="./data/training_data.csv", vocab_file="./data/training_data_vocab.json")
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
    hidden_size=256,
    num_layers=3,
    dropout=0.05,
    pad_index=dataset.pad_idx,
)
"""
model = MetroPredictionLSTM.load_from_checkpoint(
    "tb_logs/metro-delay-logger/version_1/checkpoints/epoch=120-step=3267.ckpt"
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

# trainer.test(model, test_loader)
