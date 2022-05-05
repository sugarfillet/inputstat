# inputstat 

## What is it 
inputstat is a vmstat-like tool which monitors keyboard and mouse events on linux.
in detail, collects and reports the activity statistics of keyboard and mouse. And it is written by rust.

## Demo 

```c 
 

$ sudo inputstat -k /dev/input/event5 -m /dev/input/event6 5  # print every 5 seconds 
>>  summary counts 14676 ## keyboard counts 9460 ## mouse counts 5216
>>  KEY_BACKSPACE -> 1181 ##  KEY_SPACE -> 719 ##  KEY_J -> 432 ##  KEY_LEFTCTRL -> 402 ##  KEY_I -> 376 ## 
>>  WHEEL -> 3401 ##  BTN_LEFT -> 1778 ##  BTN_RIGHT -> 30 ##  BTN_MIDDLE -> 7 ## 

>>  summary counts 14676 ## keyboard counts 9460 ## mouse counts 5216
>>  KEY_BACKSPACE -> 1181 ##  KEY_SPACE -> 719 ##  KEY_J -> 432 ##  KEY_LEFTCTRL -> 402 ##  KEY_I -> 376 ## 
>>  WHEEL -> 3401 ##  BTN_LEFT -> 1778 ##  BTN_RIGHT -> 30 ##  BTN_MIDDLE -> 7 ## 


```


## Install

```shell
cargo install inputstat
```

## How it works

1. directly communicate with event device , the userspace interface of input subsystem
2. architecture of this program
```
   k_thread  m_thread    collect_thread          display_thread 
   |______|____mpsc_____7              \_____Arc______|
```


## 
