from bs4 import BeautifulSoup

# 0 - EC Level, 1 - Numeric, 2 - AlphaNumeric, 3 - Byte, 4 - Kanji

examp = {}

file = open("raw_table.html", "r")
soup = BeautifulSoup(file.read(), "html.parser")

form1 = {
        "H": "ErrorCorrection::High",
        "M": "ErrorCorrection::Medium",
        "L": "ErrorCorrection::Low",
        "Q": "ErrorCorrection::Quartile",
}

form2 = {
        "Num": "EncodingMode::Numeric",
        "Alp": "EncodingMode::Alphanumeric",
        "Byt": "EncodingMode::Byte",
        "Kan": "EncodingMode::Kanji",
}

tab  = soup.findAll("tr")

for i in ["Num", "Alp", "Byt", "Kan"]:
    for j in ["L", "M", "Q", "H"]:
        examp[f"{i}-{j}"] = []

for i in range(len(tab)):
    row = tab[i].findAll("td")

    if len(row) == 6:
        ec_lev   = row[1].text.strip()
        examp[f"Num-{ec_lev}"].append(int(row[2].text.strip()))
        examp[f"Alp-{ec_lev}"].append(int(row[3].text.strip()))
        examp[f"Byt-{ec_lev}"].append(int(row[4].text.strip()))
        examp[f"Kan-{ec_lev}"].append(int(row[5].text.strip()))
    else:
        ec_lev   = row[0].text.strip()
        examp[f"Num-{ec_lev}"].append(int(row[1].text.strip()))
        examp[f"Alp-{ec_lev}"].append(int(row[2].text.strip()))
        examp[f"Byt-{ec_lev}"].append(int(row[3].text.strip()))
        examp[f"Kan-{ec_lev}"].append(int(row[4].text.strip()))


for i in ["Num", "Alp", "Byt", "Kan"]:
    for j in ["L", "M", "Q", "H"]:
        inled  = "    (VerKey::new("
        inled += form1[j]
        inled += ", "
        inled += form2[i]
        inled += ")"
        afslut = "),"
        print(inled, ", ", examp[f"{i}-{j}"], afslut)
