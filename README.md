<div align="center">
    <img src="logo/waffle.png">
    <p>Is a little stack-based VM built for educational purposes.</p>
</div>


## Usage

```shell script
./waffle --help
```


## How does it work?

You can find example code [here](test). 
Each instruction takes 8 bits with integers and float numbers as 64 bits (a subject to change in the future as an argument to the VM).
Allocated memory size can be managed with a cli argument. By default it's 1Mb.