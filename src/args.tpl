name:                  jubidee
version:               "0.3.48"
author:                Olaf Michaelis <o.mic@web.de>
about:                 Jubidee - calculates jubilees and birthdays
args:
    - jubidee:
        short:         j
        long:          jubidee
        help:          jubidee mode that prints a list sorted by daily jubilees
        takes_value:   false
        multiple:      false
        required:      false
    - limit:
            short:         l
            long:          limit
            help:          maximum number of final output list entries
            default_value: "0"
            value_name:    INT
            takes_value:   true
            multiple:      false
            required:      false
