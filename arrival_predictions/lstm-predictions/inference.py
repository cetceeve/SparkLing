import torch
import torch.nn as nn
import torch.utils.data as tud
import pytorch_lightning as pl
import pytorch_lightning.callbacks as plc
import pytorch_lightning.loggers as pll

from model import *
from dataset import *
import random

torch.multiprocessing.set_sharing_strategy("file_system")
torch.manual_seed(0)
random.seed(0)

# load dataset
dataset = FolkDataset(data_file="data/data_v2", max_sequence_length=256)

dataset_train, dataset_test = tud.random_split(dataset, [0.95, 0.05])

# load model
model = FolkRNN.load_from_checkpoint(
    "tb_logs/folkRNN/version_1/checkpoints/epoch=20-step=6636.ckpt"
)
model.eval()

seed = 2394
torch.manual_seed(seed)
random.seed(seed)
gen = model.inference(
    dataset.sos_idx,
    dataset.eos_idx,
    max_len=256,
    mode="topp",
    temperature=1,
    PK=0.9,
)


gen = [dataset.i2w[str(g)] for g in gen[1:-1]]
print(f"X:0\n{gen[0]}\n{gen[1]}\n{''.join(gen[2:])}")

"""
with open("results.txt", "w") as f:
    f.write(s)
"""
