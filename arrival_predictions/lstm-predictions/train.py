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
dataset = FolkDataset(data_file="data/data_v2", max_sequence_length=512)

dataset_train, dataset_val = tud.random_split(dataset, [0.95, 0.05])

batch_size = 64
train_loader = tud.DataLoader(dataset_train, batch_size=batch_size, num_workers=8)
val_loader = tud.DataLoader(dataset_val, batch_size=batch_size, num_workers=8)

"""
# load model
model = FolkRNN(
    vocab_size=dataset.vocab_size,
    hidden_size=512,
    num_layers=3,
    dropout=0.5,
    pad_index=dataset.pad_idx,
)
"""
model = FolkRNN.load_from_checkpoint(
    "tb_logs/folkRNN/version_1/checkpoints/epoch=20-step=6636.ckpt"
)

# logger
logger = pll.TensorBoardLogger("tb_logs", name="folkRNN-longer")

# training
trainer = pl.Trainer(
    accelerator="auto",
    min_epochs=10,
    max_epochs=200,
    logger=logger,
    gradient_clip_val=5,
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
