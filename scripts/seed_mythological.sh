#!/bin/bash
set -e

# Create data directory if it doesn't exist
mkdir -p data

# Check if the myth-symbol-seed.json file exists
if [ ! -f "data/myth-symbol-seed.json" ]; then
    echo "Mythological symbols file not found: data/myth-symbol-seed.json"
    echo "Creating the file..."
    cat > data/myth-symbol-seed.json << EOF
[
  {
    "id": "ares",
    "name": "Ares",
    "category": "mythology",
    "description": "Greek god of war and battle lust.",
    "interpretations": {
      "default": "Symbolizes conflict, aggression, bloodshed, and untamed masculine energy."
    },
    "related_symbols": ["war", "blood", "sword"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Warrior"
    }
  },
  {
    "id": "athena",
    "name": "Athena",
    "category": "mythology",
    "description": "Greek goddess of wisdom, strategy, and war.",
    "interpretations": {
      "default": "Represents intellect, justice, disciplined warfare, and creative strategy."
    },
    "related_symbols": ["owl", "shield", "spear"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Strategist"
    }
  },
  {
    "id": "odin",
    "name": "Odin",
    "category": "mythology",
    "description": "Norse all-father, god of wisdom, magic, and war.",
    "interpretations": {
      "default": "Symbol of sacrifice, hidden knowledge, and the shamanic quest."
    },
    "related_symbols": ["raven", "spear", "eye"],
    "properties": {
      "pantheon": "Norse",
      "archetype": "Sage"
    }
  },
  {
    "id": "thor",
    "name": "Thor",
    "category": "mythology",
    "description": "Norse god of thunder, protection, and strength.",
    "interpretations": {
      "default": "Embodies brute force, natural power, and righteous defense."
    },
    "related_symbols": ["hammer", "storm", "lightning"],
    "properties": {
      "pantheon": "Norse",
      "archetype": "Protector"
    }
  },
  {
    "id": "freya",
    "name": "Freya",
    "category": "mythology",
    "description": "Norse goddess of love, beauty, and magic.",
    "interpretations": {
      "default": "Represents sensuality, fertility, and seidr (magic)."
    },
    "related_symbols": ["cat", "gold", "cloak"],
    "properties": {
      "pantheon": "Norse",
      "archetype": "Lover"
    }
  },
  {
    "id": "anubis",
    "name": "Anubis",
    "category": "mythology",
    "description": "Egyptian god of mummification and afterlife.",
    "interpretations": {
      "default": "Symbolizes protection of the dead, transitions, and guardianship."
    },
    "related_symbols": ["jackal", "tomb", "weighing scale"],
    "properties": {
      "pantheon": "Egyptian",
      "archetype": "Guardian"
    }
  },
  {
    "id": "ra",
    "name": "Ra",
    "category": "mythology",
    "description": "Egyptian sun god, king of all gods.",
    "interpretations": {
      "default": "Embodies creation, solar vitality, divine authority."
    },
    "related_symbols": ["sun", "falcon", "disk"],
    "properties": {
      "pantheon": "Egyptian",
      "archetype": "Sun King"
    }
  },
  {
    "id": "isis",
    "name": "Isis",
    "category": "mythology",
    "description": "Egyptian goddess of healing, motherhood, and magic.",
    "interpretations": {
      "default": "Represents resurrection, divine femininity, and loyalty."
    },
    "related_symbols": ["throne", "wings", "ankh"],
    "properties": {
      "pantheon": "Egyptian",
      "archetype": "Mother"
    }
  },
  {
    "id": "vishnu",
    "name": "Vishnu",
    "category": "mythology",
    "description": "Hindu god of preservation and cosmic order.",
    "interpretations": {
      "default": "Symbol of balance, order, and dharma (cosmic law)."
    },
    "related_symbols": ["conch", "chakra", "lotus"],
    "properties": {
      "pantheon": "Hindu",
      "archetype": "Preserver"
    }
  },
  {
    "id": "shiva",
    "name": "Shiva",
    "category": "mythology",
    "description": "Hindu god of destruction and transformation.",
    "interpretations": {
      "default": "Represents endings, asceticism, and rebirth through destruction."
    },
    "related_symbols": ["trident", "third eye", "drum"],
    "properties": {
      "pantheon": "Hindu",
      "archetype": "Destroyer"
    }
  },
  {
    "id": "kali",
    "name": "Kali",
    "category": "mythology",
    "description": "Hindu goddess of time, change, and empowerment.",
    "interpretations": {
      "default": "Embodies primal energy, rage, liberation, and death."
    },
    "related_symbols": ["skull", "tongue", "fire"],
    "properties": {
      "pantheon": "Hindu",
      "archetype": "Transformer"
    }
  },
  {
    "id": "quetzalcoatl",
    "name": "Quetzalcoatl",
    "category": "mythology",
    "description": "Aztec feathered serpent deity of wind and knowledge.",
    "interpretations": {
      "default": "Symbol of duality, wisdom, and cultural renewal."
    },
    "related_symbols": ["feathers", "serpent", "wind"],
    "properties": {
      "pantheon": "Aztec",
      "archetype": "Creator"
    }
  },
  {
    "id": "amaterasu",
    "name": "Amaterasu",
    "category": "mythology",
    "description": "Shinto sun goddess, ruler of the heavens.",
    "interpretations": {
      "default": "Symbol of divine light, order, and celestial ancestry."
    },
    "related_symbols": ["mirror", "sun", "light"],
    "properties": {
      "pantheon": "Shinto",
      "archetype": "Illuminator"
    }
  },
  {
    "id": "loki",
    "name": "Loki",
    "category": "mythology",
    "description": "Norse trickster god of mischief and shapeshifting.",
    "interpretations": {
      "default": "Represents chaos, transformation, subversion of order."
    },
    "related_symbols": ["fire", "snake", "illusion"],
    "properties": {
      "pantheon": "Norse",
      "archetype": "Trickster"
    }
  },
  {
    "id": "hades",
    "name": "Hades",
    "category": "mythology",
    "description": "Greek god of the underworld and wealth.",
    "interpretations": {
      "default": "Symbol of death, mystery, and hidden power."
    },
    "related_symbols": ["underworld", "cerberus", "coin"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Shadow"
    }
  },
  {
    "id": "hermes",
    "name": "Hermes",
    "category": "mythology",
    "description": "Greek god of messages, boundaries, and commerce.",
    "interpretations": {
      "default": "Symbolizes communication, crossing thresholds, and cleverness."
    },
    "related_symbols": ["winged sandals", "staff", "paths"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Messenger"
    }
  },
  {
    "id": "artemis",
    "name": "Artemis",
    "category": "mythology",
    "description": "Greek goddess of the hunt, wilderness, and the moon.",
    "interpretations": {
      "default": "Represents independence, protection, and sacred wildness."
    },
    "related_symbols": ["bow", "moon", "stag"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Huntress"
    }
  },
  {
    "id": "horus",
    "name": "Horus",
    "category": "mythology",
    "description": "Egyptian falcon-headed god of kingship and the sky.",
    "interpretations": {
      "default": "Symbol of protection, leadership, and vision."
    },
    "related_symbols": ["eye", "falcon", "sky"],
    "properties": {
      "pantheon": "Egyptian",
      "archetype": "Sky King"
    }
  },
  {
    "id": "tlaloc",
    "name": "Tlaloc",
    "category": "mythology",
    "description": "Aztec god of rain, fertility, and water.",
    "interpretations": {
      "default": "Embodies life-giving force, fertility, and divine anger."
    },
    "related_symbols": ["rain", "storm", "jade"],
    "properties": {
      "pantheon": "Aztec",
      "archetype": "Rain Spirit"
    }
  },
  {
    "id": "chiron",
    "name": "Chiron",
    "category": "mythology",
    "description": "Greek centaur known for his wisdom and healing.",
    "interpretations": {
      "default": "Symbol of wounded healing, mentorship, and inner integration."
    },
    "related_symbols": ["bow", "wound", "medicine"],
    "properties": {
      "pantheon": "Greek",
      "archetype": "Healer"
    }
  }
]
EOF
    echo "Created mythological symbols file at data/myth-symbol-seed.json"
fi

# Run the mythological symbol seeder
echo "Running the mythological symbols seeder..."
cargo run --bin myth_seeder data/myth-symbol-seed.json

echo "Done!" 