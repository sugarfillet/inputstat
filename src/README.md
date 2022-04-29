# inputstat 

## What is it 
inputstat is a vmstat-like tool that monitors all your daily input operations on linux workstation by keyboard or mouse. 

In Detail, inputstat can count the amounts of keys and mouses. So if you are a mouseless advocate, show the tool's output to show it off.

inputstat is my hobby project written by rust. 



## Demo 

```c 

$ sudo inputstat -k /dev/input/event5 -m /dev/input/event6 5  # print every 5 seconds 
>>  summary counts 37 keyboard counts 18 ,mouse counts 19
>>  KEY_ENTER -> 1 ##  KEY_A -> 1 ##  KEY_F -> 2 ##  KEY_S -> 3 ##  KEY_LEFTCTRL -> 11 ## 
>>  WHEEL -> 2 ##  BTN_MIDDLE -> 3 ##  BTN_LEFT -> 13 ##  BTN_RIGHT -> 1 ## 

>>  summary counts 38 keyboard counts 19 ,mouse counts 19
>>  KEY_ENTER -> 2 ##  KEY_A -> 1 ##  KEY_F -> 2 ##  KEY_S -> 3 ##  KEY_LEFTCTRL -> 11 ## 
>>  WHEEL -> 2 ##  BTN_MIDDLE -> 3 ##  BTN_LEFT -> 13 ##  BTN_RIGHT -> 1 ## 

```


## Install

```shell
cargo install inputstat
```



## How it works

1. directly communicate with event device , the the userspace interface of input subsystem
2. architecture of this program
```
   k_thread  m_thread    collect_thread          display_thread 
   |______|____mpsc_____7              \_____Arc______|
```
