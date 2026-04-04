#!/data/data/com.termux/files/usr/bin/bash
# termux-install.sh - Install LyRemember APK from Termux
#
# Usage: bash termux-install.sh [path-to-apk]
#
# termux-open ne peut pas ouvrir les APK directement depuis le stockage
# prive de Termux ($HOME). Ce script copie l'APK dans le stockage partage
# (~/storage/downloads/) puis l'ouvre avec le bon type MIME.

set -e

APK_FILE="${1:-}"

# Find APK if no argument given
if [ -z "$APK_FILE" ]; then
    APK_FILE=$(ls -t LyRemember-*.apk 2>/dev/null | head -1)
    if [ -z "$APK_FILE" ]; then
        echo "Erreur: Aucun fichier APK trouve."
        echo "Usage: bash termux-install.sh <fichier.apk>"
        exit 1
    fi
    echo "APK detecte: $APK_FILE"
fi

if [ ! -f "$APK_FILE" ]; then
    echo "Erreur: Fichier '$APK_FILE' introuvable."
    exit 1
fi

# Ensure shared storage access is set up
if [ ! -d "$HOME/storage/downloads" ]; then
    echo "Configuration de l'acces au stockage partage..."
    echo "Veuillez accepter la permission quand Android la demande."
    termux-setup-storage
    sleep 2
    if [ ! -d "$HOME/storage/downloads" ]; then
        echo "Erreur: Impossible d'acceder au stockage partage."
        echo "Lancez 'termux-setup-storage' manuellement et reessayez."
        exit 1
    fi
fi

# Copy APK to shared Downloads
APK_NAME=$(basename "$APK_FILE")
DEST="$HOME/storage/downloads/$APK_NAME"
echo "Copie vers le stockage partage..."
cp "$APK_FILE" "$DEST"
echo "APK copie dans: Downloads/$APK_NAME"

# Open with Android package installer
echo "Ouverture de l'installateur Android..."
termux-open --content-type application/vnd.android.package-archive "$DEST"

echo ""
echo "Si l'installateur ne s'ouvre pas, ouvrez le fichier"
echo "depuis votre gestionnaire de fichiers: Downloads/$APK_NAME"
