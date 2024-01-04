import torch
import torch.nn as nn
import pandas as pd
import numpy as np


LEARNING_RATE = 0.01
NUM_EPOCHS = 10
INPUT_WINDOW_SIZE = 32 # based on static GTFS data, covers 82% of all scheduled trips fully


def load_dataset():
    df = pd.read_csv("../training-data/training_data.csv", index_col=["trip_id", "stop_sequence"])
    df.sort_index(inplace=True)

    # iterate trips
    xs, ys = [], []
    for trip_id, trip_df in df.groupby(level=0):
        # TODO: pad with 0s
        for i in range(0, len(trip_df) - INPUT_WINDOW_SIZE - 1):
            xs.append(trip_df.iloc[i:i+INPUT_WINDOW_SIZE])
            ys.append(trip_df.iloc[i+INPUT_WINDOW_SIZE])
    return np.array(xs), np.array(ys)


class Model(nn.Module):
    def __init__(self):
        super().__init__()
        self.input_size = 1
        self.output_size = 1
        self.num_layers = 1
        self.hidden_size = 10
        # self.lstm = nn.LSTM(self.input_size, self.hidden_size, self.num_layers, batch_first=True, dropout=0.0)
        self.rnn = nn.RNN(self.input_size, self.hidden_size, self.num_layers, batch_first=True, dropout=0.0)
        self.fc = nn.Linear(self.hidden_size, self.output_size)

    def forward(self, x):
        h0 = torch.zeros(self.num_layers, x.size(0), self.hidden_size)
        # c0 = torch.zeros(self.num_layers, x.size(0), self.hidden_size)
        out, _ = self.rnn(x, h0)
        out = self.fc(out[:, -1, :])
        return out


loss_fn = nn.MSELoss()
optimizer = torch.optim.Adam(model.parameters(), lr=LEARNING_RATE)

for epoch in range(NUM_EPOCHS):
    outputs = model(...)
    optimizer.zero_grad()
    loss = loss_fn(outputs, y_train)
    loss.backward()
    optimizer.step()

    print(f"epoch [{epoch+1}/{NUM_EPOCHS}], loss: {loss.item():.5f}")
