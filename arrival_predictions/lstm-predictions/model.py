import torch
import torch.nn as nn
import pytorch_lightning as pl
import math
from torch.nn import functional as F


class MetroPredictionLSTM(pl.LightningModule):
    def __init__(
        self,
        vocab_size: int,
        embedding_size: int,
        hidden_size: int,
        num_layers: int,
        dropout: float,
        pad_idx: int,
        skp_idx: int,
        token_to_text: dict[str: str]
    ):
        super().__init__()

        self.vocab_size = vocab_size
        self.embedding_size = embedding_size
        self.hidden_size = hidden_size
        self.num_layers = num_layers
        self.dropout = dropout
        self.pad_idx = pad_idx
        self.skp_idx = skp_idx
        self.token_to_text = token_to_text

        # Creates embedding layer with fixed weights of 1 where
        # for each token in the vocab one element of one row is 1 others are zero
        self.one_hot = nn.Embedding.from_pretrained(
            embeddings=torch.eye(self.vocab_size), freeze=True
        )

        # Make actual embedding
        self.embedding = nn.Embedding(self.vocab_size, self.embedding_size);

        self.lstm = nn.LSTM(
            input_size=self.embedding_size,
            hidden_size=hidden_size,
            num_layers=num_layers,
            batch_first=True, # needed because of the embedding
        )
        self.prediction_layer = nn.Linear(hidden_size, vocab_size)

        # self.regression_layer = nn.Linear(hidden_size, 1)

        # -1 select last dimention in our case the vocab
        self.softmax = nn.Softmax(dim=-1)

        self.loss = nn.CrossEntropyLoss(ignore_index=self.pad_idx)

        # self.loss = nn.MSELoss()

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
        logits = self.prediction_layer(lstm_out)

        # regression
        # regression = self.regression_layer(lstm_out[:,-1,:])

        return logits

    def training_step(self, train_batch, batch_idx):
        x, y = train_batch
        # swapaxes needed because
        # forward.output.dim = (batch_size, sequence_length, vocab_size)
        # loss.input.dim = (batch_size, num_targets, ...rest)
        x_hat = self.forward(x).swapaxes(1,2)
        loss = self.loss(x_hat, y)
        self.log("train_loss", loss)
        return loss

    def validation_step(self, val_batch, batch_idx):
        x, y = val_batch
        x_hat = self.forward(x).swapaxes(1,2)
        loss = self.loss(x_hat, y)
        self.log("val_loss", loss)
        self.log("hp_metric", loss)
        return loss

    def test_step(self, test_batch, batch_idx):
        x, y = test_batch
        x_hat = self.forward(x).swapaxes(1,2)
        loss = self.loss(x_hat, y)
        self.log("test_loss", loss)
        return loss

    def inference(self, sequence):
        with torch.no_grad():
            generation = []
            for idx in sequence:
                # reached end of predictable sequence
                if idx == self.pad_idx:
                    break
                # token we need to predict
                if idx == self.skp_idx:
                    # current state of generation as input
                    input_tensor = torch.Tensor(generation).to(self.device).unsqueeze(0)
                    # take prediction accoding to softmax
                    linear_output = self.forward(input_tensor)[:, -1, :]
                    # classification is unnecessary as softmax would select highest value anyway
                    # (batch_size, sequence_length, vocab_size) -> (batch_size, sequence_length, vocab_size)
                    # classification = self.softmax(linear_output)[:, -1, :]

                    # returns index of highest value in flattened output tensor
                    pred_idx = torch.argmax(linear_output);
                    generation.append(pred_idx)
            
                # all other tokens are added to the generation
                generation.append(idx)

            translation = [self.token_to_text[str(g)] for g in generation]
            return (generation, translation)

                

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
