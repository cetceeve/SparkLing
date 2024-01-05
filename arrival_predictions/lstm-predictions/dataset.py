import torch
import torch.utils.data as tud
import torch.nn.functional as F
import json
import numpy as np

from dictionary import create_vocabulary

class MetroDelayDataset(tud.Dataset):
    def __init__(self, data_file: str, vocab_file: str, feature_store):
        self.max_sequence_length = 75
        self.text_to_token = {}
        self.token_to_text = {}

        fg = feature_store.get_feature_group(
            name="metro",
            version=2,
        )
        query = fg.select_all()
        feature_view = feature_store.get_or_create_feature_view(
            name="metro",
            version=1,
            description="Read the metro sequence dataset",
            query=query
        )
        # Returns all features as training data and an empty label df
        X , _ = feature_view.training_data()
        print("Training data:")
        print(X)
        self.data = X.to_numpy(int)
        
        try:
            with open(vocab_file, "r") as f:
                data = json.load(f)
                self.text_to_token = data["text_to_token"]
                self.token_to_text = data["token_to_text"]
                print("Found dictionary.")
        except:
            print("No dictionary found.")
            vocabs = create_vocabulary()
            self.token_to_text = vocabs["token_to_text"]
            self.text_to_token = vocabs["text_to_token"]

        # with open(data_file, "r") as f:
        #     reader_obj = csv.reader(f) 
            
        #     for seq in reader_obj:
        #         # line = line.split(" ")
        #         # line = list(filter(None, line))
        #         # if len(line) >= self.max_sequence_length - 2:
        #         #     continue
        #         # seq = [self.sos_token] + [self.w2i[w] for w in line] + [self.eos_token]
        #         # length = len(seq)
        #         # seq.extend([self.pad_token] * (self.max_sequence_length - length + 1))
        #         # print(seq)
        #         self.data.append(
        #             (torch.LongTensor(seq[:-1]), torch.LongTensor(seq[1:]))
        #         )

    # @property
    # def sos_idx(self):
    #     return self.text_to_token["<sos>"]

    # @property
    # def eos_idx(self):
    #     return self.text_to_token["<eos>"]

    @property
    def pad_idx(self):
        return self.text_to_token["<pad>"]

    @property
    def vocab_size(self):
        return len(self.text_to_token)

    def __len__(self):
        return len(self.data)

    def __getitem__(self, idx):
        # training data is just shifted sequence
        return (torch.from_numpy(self.data[idx][:-1]), torch.from_numpy(self.data[idx][1:]))
