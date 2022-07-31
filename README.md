# A Discord Rich Presence for cmus player with 🦀

[![Lines of Code](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=ncloc)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Quality Gate Status](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Technical Debt](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=sqale_index)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Duplicated Lines (%)](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=duplicated_lines_density)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Vulnerabilities](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=vulnerabilities)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Bugs](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=bugs)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Code Smells](https://sonarcloud.io/api/project_badges/measure?project=anas-elgarhy_cmus-rpc-rs&metric=code_smells)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/3e0d24aa2c1441e484622b8540193cdf)](https://app.codacy.com/gh/anas-elgarhy/cmus-rpc-rs?utm_source=github.com&utm_medium=referral&utm_content=Anas-Elgarhy/cmus-rpc&utm_campaign=Badge_Grade_Settings)
[![CodeFactor](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc-rs/badge)](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc)

<img alt="image 1" src="./Screenshots/1_0.1.0.png">
<img alt="image 2" src="./Screenshots/2_0.1.0.png">

- Require cmus

<!--
## Install

### Linux
- From aur: `yay -S cmus-rpc-rs-rs`
- Manual:
  - Make sure you installed `wget`
  - Run this command
     ```bash
      curl -s https://raw.githubusercontent.com/anas-elgarhy/cmus-rpc-rs/master/scripts/install.sh | sudo bash
     ```

## Uninstall

### Linux
- Manual:
  - Run this command
    ```bash
      curl -s https://raw.githubusercontent.com/anas-elgarhy/cmus-rpc-rs/master/scripts/uninstall.sh | sudo bash
    ``` -->

### Options:

| Option                       | Description                                                  | Values                                                                                    |
| ---------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `-h` or `--help`             | Show the help                                                | -                                                                                         |
| `-V` or `--version`          | Show the version                                             | -                                                                                         |
| `-d` or `--debug`            | Debug mode                                                   | -                                                                                         |
| `-l` or `--link`             | Linking with cmus (close the program if cmus is not running) | -                                                                                         |
| `-c` or `--config`           | Set custom path to config file                               | Path to config file .json                                                                 |
| `-i` or `--interval`         | Set interval between checks                                  | interval time (seconds)                                                                   |
| `-s` or `--sleep`            | Set sleep when there is no activity                          | sleep time (seconds)                                                                      |
| `--p1f` or `--partOneFormat` | Set the format for the first part                            | Format for first part                                                                     |
| `--p2f` or `--partTowFormat` | Set the format for the second part                           | Format for second part                                                                    |
| `--li` or `--largeImage`     | Set custom large image for presence                          | large imgae name [Available images for defult app]()                                      |
| `--pi` or `--playingImage`   | Set custom playing image for presence                        | playing icon name [Available playing images for defult app]()                             |
| `--pai` or `--pausedImage`   | Set custom paused image for presence                         | paused icon name [Available icons]()                                                      |
| `--pt` or `--playingText`    | Set custom playing icon alt for presence                     | playing icon alt text                                                                     |
| `--pat` or `--pausedText`    | Set custom paused icon altarnative for presence              | paused icon alt text                                                                      |
| `--b1t` or `--buttonOneText` | Set button one text(label)                                   | button one label (if empty and no has value in the config file the button will be hidden) |
| `--b1u` or `--buttonOneUrl`  | Set button one url                                           | Button one url (if empty and no has value in the config file the button will be hidden)   |
| `--b2t` or `--buttonTwoText` | Set button two text(label)                                   | button one label (if empty and no has value in the config file the button will be hidden) |
| `--b2u` or `--buttonTwoUrl`  | Set button two url                                           | Button one url (if empty and no has value in the config file the button will be hidden)   |

### Examples:

```bash
cmus-rpc-rs --p1f %title%
```

```bash
cmus-rpc-rs --p1f "%artist% - %title%" --p2f "%album% - %date%"
```

```bash
cmus-rpc-rs --p1f "Anas listening to %title%" --p2f "From %artist%"
```

### How to do auto run when you start the cmus

- Put the following line in your shellrc file e.g. `.bashrc` or `.zshrc`

```bash
    alias cmus = 'cmus-rpc-rs --link &>/dev/null & cmus'
```

### Available in

[![GitHub](https://img.shields.io/badge/GitHub-Main%20repo-brightgreen?style=for-the-badge&logo=GitHub)](https://github.com/anas-elgarhy/cmus-rpc-rs)
[![GitLab](https://img.shields.io/badge/GitLab-Mirror%20repo-brightgreen?style=for-the-badge&logo=GitLab)](https://gitlab.com/anas-elgarhy/cmus-rpc-rs)
[![BitBucket](https://img.shields.io/badge/BitBucket-Mirror%20repo-brightgreen?style=for-the-badge&logo=BitBucket)](https://bitbucket.org/anas_elgarhy/cmus-rpc-rs)
[![Codeberg](https://img.shields.io/badge/Codeberg-Mirror%20repo-brightgreen?style=for-the-badge&logo=Codeberg)](https://codeberg.org/anas-elgarhy/cmus-rpc-rs)

### Recourses

- [`cmus-remote` tool](https://github.com/cmus/cmus) to the make a query to the cmus
- [Discord Rich Presence](https://github.com/nickofolas/discord-rich-presence) A simple, cross-platform crate for interfacing with Discord's IPC.
- [dirs-rs](https://github.com/dirs-dev/dirs-rs) a low-level library that provides config/cache/data paths, following the respective conventions on Linux, macOS and Windows.
- [clap](https://github.com/clap-rs/clap) A full featured, fast Command Line Argument Parser for Rust.

[![Quality gate](https://sonarcloud.io/api/project_badges/quality_gate?project=anas-elgarhy_cmus-rpc-rs)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)

[![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-black.svg)](https://sonarcloud.io/summary/new_code?id=anas-elgarhy_cmus-rpc-rs)

[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://spdx.org/licenses/MIT.html)
