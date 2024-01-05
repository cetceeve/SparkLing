import json

# Pretty print the dictionary with an indentation of 4 spaces
def pretty_dict(my_dict):
    return json.dumps(my_dict, ensure_ascii=False, indent=2)
# A token in our Vocabulary.
# 0-30: time_delta -15m to +15m
# 31: sos
# 32: eos
# 33: pad
# 34-47: route_token: route_id + direction_id
# 48-149: stop_name ['Kungsträdgården', 'T-Centralen', 'Rådhuset', 'Fridhemsplan', 'Stadshagen', 'Västra skogen', 'Huvudsta', 'Solna strand', 'Sundbybergs centrum', 'Duvbo', 'Rissne', 'Rinkeby', 'Tensta', 'Hjulsta', 'Solna centrum', 'Näckrosen', 'Hallonbergen', 'Kymlinge norrut', 'Kista', 'Husby', 'Akalla', 'Kymlinge söderut', 'Norsborg', 'Hallunda', 'Alby', 'Fittja', 'Masmo', 'Vårby gård', 'Vårberg', 'Skärholmen', 'Sätra', 'Bredäng', 'Mälarhöjden', 'Axelsberg', 'Örnsberg', 'Aspudden', 'Liljeholmen', 'Hornstull', 'Zinkensdamm', 'Mariatorget', 'Slussen', 'Gamla stan', 'Östermalmstorg', 'Karlaplan', 'Gärdet', 'Ropsten', 'Mörby centrum', 'Danderyds sjukhus', 'Bergshamra', 'Universitetet', 'Tekniska högskolan', 'Stadion', 'Fruängen', 'Västertorp', 'Hägerstensåsen', 'Telefonplan', 'Midsommarkransen', 'Skarpnäck', 'Bagarmossen', 'Kärrtorp', 'Björkhagen', 'Hammarbyhöjden', 'Skärmarbrink', 'Gullmarsplan', 'Skanstull', 'Medborgarplatsen', 'Hötorget', 'Rådmansgatan', 'Odenplan', 'S:t Eriksplan', 'Thorildsplan', 'Kristineberg', 'Alvik', 'Stora mossen', 'Abrahamsberg', 'Brommaplan', 'Åkeshov', 'Ängbyplan', 'Islandstorget', 'Blackeberg', 'Råcksta', 'Vällingby', 'Johannelund', 'Hässelby gård', 'Hässelby strand', 'Farsta strand', 'Farsta', 'Hökarängen', 'Gubbängen', 'Tallkrogen', 'Skogskyrkogården', 'Sandsborg', 'Blåsut', 'Hagsätra', 'Rågsved', 'Högdalen', 'Bandhagen', 'Stureby', 'Svedmyra', 'Sockenplan', 'Enskede gård', 'Globen']
# 150-156: weekday
# 157-180: hour_of_day
def create_vocabulary(): 
    token_to_text = {}
    # 0-30: time_delta -15m to +15m
    for token in range(0,15):
        token_to_text[token] = "d-" + str(15-token)
    for token in range(15,31):
        token_to_text[token] = "d+" + str(token-15)
    # 31: sos
    # 32: eos
    # 33: pad
    token_to_text[31] = "<sos>"
    token_to_text[32] = "<eos>"
    token_to_text[33] = "<pad>"
    # 34-47: route_token: route_id + direction_id
    routes = [
        "9011001001000000",
        "9011001001100000",
        "9011001001300000",
        "9011001001400000",
        "9011001001700000",
        "9011001001800000",
        "9011001001900000"
    ]
    for route_idx, route in enumerate(routes):
        token = 34 + route_idx * 2
        token_to_text[token] = route + "|0"
        token_to_text[token + 1] = route + "|1"
    # 48-149: stop_name ['Kungsträdgården', 'T-Centralen', 'Rådhuset', 'Fridhemsplan', 'Stadshagen', 'Västra skogen', 'Huvudsta', 'Solna strand', 'Sundbybergs centrum', 'Duvbo', 'Rissne', 'Rinkeby', 'Tensta', 'Hjulsta', 'Solna centrum', 'Näckrosen', 'Hallonbergen', 'Kymlinge norrut', 'Kista', 'Husby', 'Akalla', 'Kymlinge söderut', 'Norsborg', 'Hallunda', 'Alby', 'Fittja', 'Masmo', 'Vårby gård', 'Vårberg', 'Skärholmen', 'Sätra', 'Bredäng', 'Mälarhöjden', 'Axelsberg', 'Örnsberg', 'Aspudden', 'Liljeholmen', 'Hornstull', 'Zinkensdamm', 'Mariatorget', 'Slussen', 'Gamla stan', 'Östermalmstorg', 'Karlaplan', 'Gärdet', 'Ropsten', 'Mörby centrum', 'Danderyds sjukhus', 'Bergshamra', 'Universitetet', 'Tekniska högskolan', 'Stadion', 'Fruängen', 'Västertorp', 'Hägerstensåsen', 'Telefonplan', 'Midsommarkransen', 'Skarpnäck', 'Bagarmossen', 'Kärrtorp', 'Björkhagen', 'Hammarbyhöjden', 'Skärmarbrink', 'Gullmarsplan', 'Skanstull', 'Medborgarplatsen', 'Hötorget', 'Rådmansgatan', 'Odenplan', 'S:t Eriksplan', 'Thorildsplan', 'Kristineberg', 'Alvik', 'Stora mossen', 'Abrahamsberg', 'Brommaplan', 'Åkeshov', 'Ängbyplan', 'Islandstorget', 'Blackeberg', 'Råcksta', 'Vällingby', 'Johannelund', 'Hässelby gård', 'Hässelby strand', 'Farsta strand', 'Farsta', 'Hökarängen', 'Gubbängen', 'Tallkrogen', 'Skogskyrkogården', 'Sandsborg', 'Blåsut', 'Hagsätra', 'Rågsved', 'Högdalen', 'Bandhagen', 'Stureby', 'Svedmyra', 'Sockenplan', 'Enskede gård', 'Globen']
    stops = [
        "Kungsträdgården", "T-Centralen", "Rådhuset", "Fridhemsplan", "Stadshagen",
        "Västra skogen", "Huvudsta", "Solna strand", "Sundbybergs centrum", "Duvbo",
        "Rissne", "Rinkeby", "Tensta", "Hjulsta", "Solna centrum", "Näckrosen",
        "Hallonbergen", "Kymlinge norrut", "Kista", "Husby", "Akalla",
        "Kymlinge söderut", "Norsborg", "Hallunda", "Alby", "Fittja", "Masmo",
        "Vårby gård", "Vårberg", "Skärholmen", "Sätra", "Bredäng", "Mälarhöjden",
        "Axelsberg", "Örnsberg", "Aspudden", "Liljeholmen", "Hornstull", "Zinkensdamm",
        "Mariatorget", "Slussen", "Gamla stan", "Östermalmstorg", "Karlaplan", "Gärdet",
        "Ropsten", "Mörby centrum", "Danderyds sjukhus", "Bergshamra", "Universitetet",
        "Tekniska högskolan", "Stadion", "Fruängen", "Västertorp", "Hägerstensåsen",
        "Telefonplan", "Midsommarkransen", "Skarpnäck", "Bagarmossen", "Kärrtorp",
        "Björkhagen", "Hammarbyhöjden", "Skärmarbrink", "Gullmarsplan", "Skanstull",
        "Medborgarplatsen", "Hötorget", "Rådmansgatan", "Odenplan", "S:t Eriksplan",
        "Thorildsplan", "Kristineberg", "Alvik", "Stora mossen", "Abrahamsberg",
        "Brommaplan", "Åkeshov", "Ängbyplan", "Islandstorget", "Blackeberg", "Råcksta",
        "Vällingby", "Johannelund", "Hässelby gård", "Hässelby strand", "Farsta strand",
        "Farsta", "Hökarängen", "Gubbängen", "Tallkrogen", "Skogskyrkogården", "Sandsborg",
        "Blåsut", "Hagsätra", "Rågsved", "Högdalen", "Bandhagen", "Stureby", "Svedmyra",
        "Sockenplan", "Enskede gård", "Globen"
    ]
    for stop_idx, stop in enumerate(stops):
        token_to_text[48 + stop_idx] = stop
    # 150-156: weekday
    days_of_week = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]
    for index, day in enumerate(days_of_week):
        token_to_text[150 + index] = day

    # 157-180: hour_of_day
    for hour in range(0,24):
        token_to_text[157 + hour] = "h" + str(hour)
    
    # print(pretty_dict(token_to_meaning))

    # Reverse dictionary
    text_to_token = {value: key for key, value in token_to_text.items()}
    return {
        "token_to_text": token_to_text,
        "text_to_token": text_to_token
    }

# def main():
#     vocabs = create_vocabulary()
#     with open(f"data/training_data_vocab.json", "w") as df:
#         json.dump(vocabs, df, ensure_ascii=False)
#         print("Saved Vocabulary.")

# if __name__ == "__main__":
#     main()
