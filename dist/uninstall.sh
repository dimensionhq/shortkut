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
    echo "${red}error: ${yellow}uninstallation script requires ${cyan}\`${magenta}sudo${cyan}\`${yellow} permissions${reset}"
    exit
fi

if [[ "$OSTYPE" =~ ^darwin ]]; then
    echo "Uninstalling ${cyan}Shortkut${reset} 😞"
    echo "You will need to manually remove added shortkuts from your bash or zsh configuration files"
    while true; do
        read -p "Are you certain you want to uninstall Shortkut?" yn
        case $yn in
            [Yy]* ) sudo rm /usr/local/bin/shortkut; break;;
            [Nn]* ) exit;;
            * ) echo "Please answer yes or no.";;
        esac
    done
fi

if [[ "$OSTYPE" =~ ^linux ]]; then
    echo "Uninstalling ${cyan}Shortkut${reset} 😞"
    echo "${yellow}You will need to manually remove added shortkuts from your bash or zsh configuration files.${reset}"
    while true; do
        read -p "Are you certain you want to uninstall Shortkut (y/n): " yn
        case $yn in
            [Yy]* ) sudo rm /usr/local/bin/shortkut; break;;
            [Nn]* ) exit;;
            * ) echo "Please answer yes or no.";;
        esac
    done

    echo "Successfully Uninstalled ${cyan}Shortkut${reset}"
fi

