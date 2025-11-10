# This file is made for generation of the large amount of card info needed by Parse Royale.
#

import os
import re
import requests

from dotenv import load_dotenv

# Set up authentification.
load_dotenv()
token = os.getenv("TOKEN")
headers = { "Authorization": f"Bearer {token}" }

# Make request and verify the response.
response = requests.get("https://api.clashroyale.com/v1/cards", headers=headers)

if response.status_code != 200:
    print("Request got error code {response.status_code}")
    quit()
#

data = response.json()
if data.get("reason") is not None:
    print("PI gave an error with the reason \"{str(data['reason'])}\"")
    quit()
#

print("Got card data")

# Not all cards have their mastery with the same name (or proper pascal case) so they must be mapped.
card_mastery_map = {
    "Bandit": "MasteryAssassin",
    "Barbarian Barrel": "MasteryBarbLog",
    "Cannon Cart": "MasteryMovingCannon",
    "Dart Goblin": "MasteryBlowdartGoblin",
    "Elite Barbarians": "MasteryAngryBarbarians",
    "Elixir Collector": "MasteryElixir Collector",
    "Executioner": "MasteryAxeMan",
    "Flying Machine": "MasteryDartBarrell",
    "Furnace": "MasteryFirespiritHut",
    "Giant Snowball": "MasterySnowball",
    "Guards": "MasterySkeletonWarriors",
    "Heal Spirit": "MasteryHeal",
    "Ice Golem": "MasteryIceGolemite",
    "Ice Spirit": "MasteryIceSpirits",
    "Lumberjack": "MasteryRageBarbarian",
    "Magic Archer": "MasteryEliteArcher",
    "Mini P.E.K.K.A.": "MasteryMiniPekka",
    "Mother Witch": "MasteryWitchMother",
    "Night Witch": "MasteryDarkWitch",
    "P.E.K.K.A.": "MasteryPekka",
    "Royal Ghost": "MasteryGhost",
    "Rune Giant": "MasteryGiantBuffer",
    "Skeleton Barrel": "MasterySkeletonBalloon",
    "Sparky": "MasteryZapMachine",
    "Spirit Empress": "MasteryMergeMaiden",
    "Void": "MasteryDarkMagic",
    "Zappies": "MasteryMiniSparkys",
}

def caps_to_kebab(s: str) -> str:
    """Convert card name from Capital Case to kebab-case"""
    
    if s == "P.E.K.K.A":
        return "pekka"
    elif s == "Mini P.E.K.K.A":
        return "mini-pekka"
    
    words = re.findall(r"[A-Za-z]+", s)
    return '-'.join(word.lower() for word in words)
#

def caps_to_pascal(s: str) -> str:
    """Convert card name from Capital Case to PascalCase"""
    
    if s == "P.E.K.K.A":
        return "Pekka"
    elif s == "Mini P.E.K.K.A":
        return "MiniPekka"

    words = re.findall(r"[A-Za-z]+", s)
    return ''.join(word.capitalize() for word in words)
#

def caps_to_screaming_snake(s: str) -> str:
    """Convert card name from Capital Case to SCREAMING_SNAKE_CASE"""
    
    if s == "P.E.K.K.A":
        return "PEKKA"
    elif s == "Mini P.E.K.K.A":
        return "MINI_PEKKA"
    
    words = re.findall(r"[A-Za-z]+", s)
    return '_'.join(word.upper() for word in words)
#

with open("def.txt", "w") as f, open("match.txt", "w") as g:
    f.write("def_cards!(\n")

    for card in data["items"]:
        name = str(card["name"])
        id_ = int(card["id"])
        var_name = caps_to_screaming_snake(name)
        arg_name = caps_to_kebab(name)
        badge_name = ""

        if name in card_mastery_map:
            badge_name = card_mastery_map[name]
        else:
            badge_name = "Mastery" + caps_to_pascal(name)

        f.write(f"\t{var_name}: \"{name}\", \"{badge_name}\", {id_};\n")
        g.write(f"\"{arg_name}\" => &Self::{var_name},\n")
    #

    f.write(");")
#

print("Wrote output to `def.txt` and `match.txt`")
