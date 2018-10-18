#!/bin/bash
IFS=$'\n'
scriptdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )/"

arg="${1}"
if [[ -z "${arg}" ]]; then
    echo -e "\nPlease specify an input vcf file.\n"
    exit
fi

tf="${scriptdir}/jubidee.yaml"

arr=($(cat "${arg}" | grep -i "FN:\|BDAY:"))

v=""
for ((i=0; i<${#arr[@]}; i++)); do
    cel=$(echo ${arr[${i}]})
    nel=$(echo ${arr[$i+1]})

    if [[ "${cel:0:5}" == "BDAY:" ]] && [[ "${nel:0:3}" == "FN:" ]]; then
        bday=$(echo ${cel:5:8})
        # select by negative character set, select all but...
        name=$(echo ${nel:3:99} | grep -Po "[^\n\r:,;]+" | tr '\n' ' ')
        v+="\"${name:0:-1}\": ${bday}\n"


    fi
done
echo -e "${v}" > "${tf}"

# validate
yq read "${tf}"
if [[ "${?}" == "0" ]]; then
    echo -e "\nAll good.\nFinal dataset written here: \"${tf}\"\n"
else
    echo -e "\nError. Final yaml file is invalid."
fi
