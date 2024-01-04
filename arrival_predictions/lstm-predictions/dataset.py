import torch
import torch.utils.data as tud
import torch.nn.functional as F
from tqdm import tqdm
import json


class FolkDataset(tud.Dataset):
    def __init__(self, data_file: str, max_sequence_length: int):
        self.w2i = {}
        self.i2w = {}
        self.data = []
        self.max_sequence_length = max_sequence_length

        create_dict = False
        try:
            with open(f"{data_file}_vocab.json", "r") as f:
                data = json.load(f)
                self.w2i = data["w2i"]
                self.i2w = data["i2w"]
                print("Found dictionary.")
        except:
            print("No dictionary found.")
            create_dict = True

        with open(data_file, "r") as f:
            data = f.read().split("\n")

            if create_dict:
                print("Creating...")
                self.add_token("<sos>")
                self.add_token("<eos>")
                self.add_token("<pad>")

                for line in tqdm(data):
                    line = line.split(" ")
                    line = list(filter(None, line))
                    if len(line) >= self.max_sequence_length - 2:
                        continue
                    for tok in line:
                        if tok in self.w2i:
                            continue
                        self.add_token(tok)
                vocabs = {"w2i": self.w2i, "i2w": self.i2w}

                with open(f"{data_file}_vocab.json", "w") as df:
                    json.dump(vocabs, df)
                    print("Saved.")

            for line in tqdm(data):
                line = line.split(" ")
                line = list(filter(None, line))
                if len(line) >= self.max_sequence_length - 2:
                    continue
                seq = [self.sos_idx] + [self.w2i[w] for w in line] + [self.eos_idx]
                length = len(seq)
                seq.extend([self.pad_idx] * (self.max_sequence_length - length + 1))
                self.data.append(
                    (torch.LongTensor(seq[:-1]), torch.LongTensor(seq[1:]))
                )

    def add_token(self, tok):
        ID = len(self.w2i)
        self.w2i[tok] = ID
        self.i2w[ID] = tok

    @property
    def sos_idx(self):
        return self.w2i["<sos>"]

    @property
    def eos_idx(self):
        return self.w2i["<eos>"]

    @property
    def pad_idx(self):
        return self.w2i["<pad>"]

    @property
    def vocab_size(self):
        return len(self.w2i)

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        return self.data[idx]
