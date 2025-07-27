with open("native_english_words.txt", "r", encoding="utf-8") as f:
    for i, line in enumerate(f):
        word = line.strip()
        print(f"{i},{word}")
