# libppm

## Group
This project was made by **Kamel MALKI, Hedi BOUFADEN and Ilès BENKOUSSA**
 who are forming the group **n°10** of **class 1 IABD** on myges
 
## Documentation 
Documentation can be found in libppm/ppm/target/doc

## Run the program
- First you need to install nightly features to use the program (see at the end the reason)
```
rustup toolchain install nightly
```
- Go in the main folder
- Type this command to go in main directory :
```
cd main
```
- Build the sources :
```
cargo +nightly build
```
- Run the executable :
```
cargo +nightly run
```

## Run the tests
 You can run tests on some functions provided in the lib, to do so :
 - go in ppm directory :
 ```
 cd ppm

```
- then type :
 ```
cargo +nightly test
```
It will run tests + benchs

If you want to run only bench type :

 ```
cargo +nightly bench
```
**N.B** : +nightly parameter is necessary because we are using **#![feature(test)]** which is not available in the stable version !

**N.B 2** : Benchs can be very long depending of the file's size so they are commented by default and it's necessary to change the code manually to give the path of the image to treat
