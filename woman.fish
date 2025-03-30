# # Actions principales
complete -c woman -n '__fish_use_subcommand' -a "--status"      -d "Afficher l'état des services"
complete -c woman -n '__fish_use_subcommand' -a "--attach"      -d "Attacher à une session"
complete -c woman -n '__fish_use_subcommand' -a "--test"        -d "Lancer les outils de test"
complete -c woman -n '__fish_use_subcommand' -a "--audit"       -d "Lancer les outils de sécurité"
complete -c woman -n '__fish_use_subcommand' -a "--deps"        -d "Vérifier les dépendances"
complete -c woman -n '__fish_use_subcommand' -a "--perfs"       -d "Lancer les outils de performance"
complete -c woman -n '__fish_use_subcommand' -a "--doc"         -d "Générer la documentation"
complete -c woman -n '__fish_use_subcommand' -a "--vcs"         -d "Outils de versioning (Git)"
complete -c woman -n '__fish_use_subcommand' -a "--all"         -d "Lancer tous les outils"
complete -c woman -n '__fish_use_subcommand' -a "--restart"     -d "Redémarrer un groupe"
complete -c woman -n '__fish_use_subcommand' -a "--stop"        -d "Arrêter toutes les sessions"
complete -c woman -n '__fish_use_subcommand' -a "--clean"       -d "Nettoyer les ttyd et sessions"

# # Groupes possibles après --restart
complete -c woman -n '__fish_seen_subcommand_from --restart' -a "test audit deps perfs doc vcs all" -d "Groupe à redémarrer"

# # Services disponibles après --attach
complete -c woman -n '__fish_seen_subcommand_from --attach' -a "test clippy hack udeps audit deny outdated flamegraph doc fmt lazygit ungit" -d "Nom de la session"

# # Options de backend
complete -c woman -l tmux   -d "Utiliser tmux comme backend"
complete -c woman -l screen -d "Utiliser screen comme backend"

# # Options d'éditeur visuel
complete -c woman -l broot  -d "Lancer broot à la fin"
complete -c woman -l ranger -d "Lancer ranger à la fin"
complete -c woman -l code   -d "Lancer VS Code à la fin"

