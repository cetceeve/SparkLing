import pandas as pd
import xgboost
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error, mean_absolute_error, accuracy_score, max_error


# sequences: sos route day hour stop delta stop delta
df = pd.read_csv("training_data.csv", header=None)


# create X and Y
X = []
Y = []
for i in range(len(df)):
    row = df.loc[i]
    j = 7
    while j < len(row) and row[j] != 33:
        x = list(row)
        Y.append(float(x[j] - 15))
        # Y.append(x[j])
        for q in range(j, len(x)):
            x[q] = 33
        X.append(x)
        j += 2

X = pd.DataFrame(X)
Y = pd.Series(Y)

X_train, X_test, y_train, y_test = train_test_split(X, Y, test_size=0.3, random_state=0)

model = xgboost.XGBRegressor()
# model = xgboost.XGBClassifier()
model.fit(X_train, y_train)

y_pred = model.predict(X_test)

print(list(y_pred))
print(mean_squared_error(y_test, y_pred))
print(mean_absolute_error(y_test, y_pred))
print(max_error(y_test, y_pred))
# print(accuracy_score(y_test, y_pred))
print(max(y_pred))
print(min(y_pred))
print(y_pred.mean())


err_df = pd.DataFrame(zip(y_test, y_pred))
err_df["err"] = err_df[0] - err_df[1]
err_df = err_df.sort_values(by="err", ascending=False)
print(err_df.head(30))

