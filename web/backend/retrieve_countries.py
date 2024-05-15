import pandas as pd


def main():
    emoji = pd.read_html("https://apps.timwhitlock.info/emoji/tables/iso3166")[0]
    emoji.columns = ["ISO", "emoji", "unicode", "name"]
    emoji.fillna("NA", inplace=True)

    # replace name of CD in iso for Democratic Republic of the Congo
    emoji.loc[emoji["ISO"] == "CD", "name"] = "Democratic Republic of the Congo"
    emoji.loc[emoji["ISO"] == "CG", "name"] = "Republic of the Congo"
    kosovo = pd.DataFrame(
        [["XK", "🇽🇰", "U+1F1FD U+1F1F0", "Kosovo"]],
        columns=["ISO", "emoji", "unicode", "name"],
    )
    emoji = pd.concat([emoji, kosovo], ignore_index=True)
    emoji.to_csv("db_data/countries.csv", index=False)


if __name__ == "__main__":
    main()
