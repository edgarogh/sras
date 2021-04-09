# Service de Réanimation de l'Arbre du Salon

_a.k.a projet plante connectée @ Polytech_

## Arduino

Le projet est essentiellement un projet [Arduino](https://arduino.cc). Le code peut être trouvé dans `/arduino/sras/`.

Bibliothèques utilisées:
  * pcd8544 de Carlos Rodrigues – https://github.com/carlosefr/pcd8544

## CLI

L'interface en ligne de commande permet d'émuler un écran PCD8544 de façon relativement fiable<sup>*</sup>, mais également d'avoir accès facilement à la commande de la pompe, aux capteurs de l'Arduino et calibrer ceux-ci.

L'autocomplétion [`fish`](https://fishshell.com/) peut être trouvée dans `/cli/sras-cli.fish`.

## License

Le projet est sous license GPL-3.0. Le fichier `/cli/src/charset.hpp.rs` est tiré du fichier [`/charset.cpp`](https://github.com/carlosefr/pcd8544/blob/master/charset.cpp), également de Carlos Rodrigues, à l'origine sous license MIT (`Copyright (c) 2013 Carlos Rodrigues <cefrodrigues@gmail.com>`), que je laisse sous cette license pour plus de simplicité.

---

<sup>*</sup><small>...comparé à un écran qui ne marche pas. Le protocole est bancal au possible et facile à casser.</small>
