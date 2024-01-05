import torch
import torch.nn as nn
import pytorch_lightning as pl
import math
from torch.nn import functional as F


class MetroPredictionLSTM(pl.LightningModule):
    def __init__(
        self,
        vocab_size: int,
        hidden_size: int,
        num_layers: int,
        dropout: float,
        pad_index: int,
    ):
        super().__init__()

        self.vocab_size = vocab_size
        self.hidden_size = hidden_size
        self.num_layers = num_layers
        self.dropout = dropout

        # Creates embedding layer with fixed weights of 1 where
        # for each token in the vocab one element of one row is 1 others are zero
        self.one_hot = nn.Embedding.from_pretrained(
            embeddings=torch.eye(self.vocab_size), freeze=True
        )

        # Make actual embedding
        self.embedding = nn.Embedding(self.vocab_size, 64);

        # 
        self.lstm = nn.LSTM(
            input_size=64,
            hidden_size=hidden_size,
            num_layers=num_layers,
            batch_first=True, # needed because of the embedding
        )
        self.prediction_layer = nn.Linear(hidden_size, vocab_size)

        self.regression_layer = nn.Linear(hidden_size, 1)

        # -1 select last dimention in our case the vocab
        self.softmax = nn.Softmax(dim=-1)

        # self.loss = nn.CrossEntropyLoss(ignore_index=pad_index)

        self.loss = nn.MSELoss()

        self.save_hyperparameters()

    # (batch_size, sequence_length)
    def forward(self, X):
        # one-hot
        # (batch_size, sequence_length) -> (batch_size, sequence_length, vocab_size)
        embedded_input = self.embedding(X)

        # LSTM
        # (batch_size, sequence_length, vocab_size) -> (batch_size, sequence_length, hidden_size)
        lstm_out, _ = self.lstm(embedded_input)

        # prediction
        # (batch_size, sequence_length, hidden_size) -> (batch_size, sequence_length, vocab_size)
        # logits = self.prediction_layer(lstm_out)

        # regression
        regression = self.regression_layer(lstm_out[:,-1,:])

        # classification
        # (batch_size, sequence_length, vocab_size) -> (batch_size, sequence_length, vocab_size)
        # classification = self.softmax(logits)
        return regression

    def training_step(self, train_batch, batch_idx):
        x, y = train_batch
        # swapaxes needed because
        # forward.output.dim = (batch_size, sequence_length, vocab_size)
        # loss.input.dim = (batch_size, num_targets, ...rest)
        x_hat = self.forward(x)
        loss = self.loss(x_hat, y)
        self.log("train_loss", loss)
        return loss

    def validation_step(self, val_batch, batch_idx):
        x, y = val_batch
        x_hat = self.forward(x)
        loss = self.loss(x_hat, y)
        self.log("val_loss", loss)
        self.log("hp_metric", loss)
        return loss

    def test_step(self, test_batch, batch_idx):
        x, y = test_batch
        x_hat = self.forward(x)
        loss = self.loss(x_hat, y)
        self.log("test_loss", loss)
        return loss

    # def inference(
    #     self,
    #     sos_idx,
    #     eos_idx,
    #     max_len=100,
    #     mode="greedy",
    #     temperature=1,
    #     PK=1,
    # ):
    #     with torch.no_grad():
    #         generation = [sos_idx]
    #         t = 0
    #         while t < max_len:
    #             input = torch.LongTensor(generation).to(self.device).unsqueeze(0)

    #             logits = self.forward(input)[:, -1, :]

    #             tok = self.sample(logits, mode=mode, T=temperature, K=PK, P=PK)
    #             generation.append(tok)

    #             if tok == eos_idx:
    #                 break
    #             t += 1

    #         return generation

    # def sample(self, out, mode="greedy", K=5, T=1, P=0.9):
    #     if mode == "greedy":
    #         sample = torch.argmax(out)

    #     elif mode == "topk":
    #         values, indexes = torch.topk(out, K, dim=-1)
    #         out = out.clone().squeeze(1)
    #         out[out < values[:, -1]] = -float("Inf")
    #         probs = self.softmax(out / T).squeeze()
    #         sample = torch.multinomial(probs, 1)

    #     elif mode == "topp":
    #         values, indexes = torch.sort(out / T, descending=True)
    #         values = self.softmax(values)
    #         cum_probs = torch.cumsum(values, dim=-1)

    #         remove = cum_probs > P
    #         remove[..., 1:] = remove[..., :-1].clone()
    #         remove[..., 0] = 0

    #         out = out.clone()
    #         remove = torch.zeros_like(out, dtype=torch.bool).scatter_(
    #             dim=-1, index=indexes, src=remove
    #         )
    #         out[remove] = -float("Inf")

    #         probs = self.softmax(out / T).squeeze()
    #         sample = torch.multinomial(probs, 1)

    #     return sample.item()

    def configure_optimizers(self):
        optimizer = torch.optim.AdamW(self.parameters(), lr=5 * 1e-3)
        return optimizer
