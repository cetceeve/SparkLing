import joblib
import numpy as np

model = joblib.load("./torch_tf_model/metro_delay_lstm.pkl")

test_input = np.array([31,44,151,165,100,15,101,15,102,15,103,15,104,181,84,181,85,181,86,181,87,181,88,181,89,181,49,181,90,181,99,181,98,181,97,181,96,181,95,181,94,181,33])
pred= model.inference(test_input)

print(pred)
