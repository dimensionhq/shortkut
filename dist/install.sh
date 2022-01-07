# /bin/bash

black=`tput setaf 0`
red=`tput setaf 1`
green=`tput setaf 2`
yellow=`tput setaf 3`
blue=`tput setaf 4`
magenta=`tput setaf 5`
cyan=`tput setaf 6`
white=`tput setaf 7`
reset=`tput sgr0`


if [[ $(/usr/bin/id -u) -ne 0 ]]; then
    echo "${red}error: ${yellow}installation script requires ${cyan}\`${magenta}sudo${cyan}\`${yellow} permissions${reset}"
    exit
fi

if [[ "$OSTYPE" =~ ^darwin ]]; then
    echo "Downloading Shortkut for MacOS..."
    curl -# -o /tmp/shortkut.macos "https://cdn.xtremedevx.com/dl/shortkut/shortkut.macos"
    sudo mv /tmp/shortkut.macos /usr/local/bin/shortkut
fi

if [[ "$OSTYPE" =~ ^linux ]]; then
    echo "Downloading ${cyan}Shortkut${reset} for Linux 🐧"
    curl -# -o /tmp/shortkut.linux "http://xtreme-cdn.herokuapp.com/dl/shortkut/shortkut.linux"
    sudo mv /tmp/shortkut.linux /usr/local/bin/shortkut
    echo "Installing ${cyan}Shortkut${reset}"
    sudo chmod +x /usr/local/bin/shortkut
    echo "${green}Successfully Installed Shortkut${reset}"
    echo "To use Shortkut, type \`${cyan}shortkut ${magenta}--help${reset}\` in your terminal"
fi

