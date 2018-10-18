# Jubidee
Calculate which birthdays are next.

## Automatically create a config yaml
In the `tools` folder there is a simple shell script, that is able to automatically create a yaml config file based on a vcf addressbook. All people's entries in the addressbook will be parsed and put into a dictionary. Finally all this is written into `jubidee.yaml`.

The necessary vcf can for example be grabbed from Radicale or exported out of Thunderbird.
```shell
parse-vcf.sh addressbook.vcf
```
