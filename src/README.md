# inputstat 
## What is it 
inputstat is a vmstat-like tool that monitors all your daily input operations on linux workstation by keyboard or mouse. 

In Detail, inputstat can count the amounts of keys and mouses. So if you are a mouseless advocate, show the tool's output to show it off.

inputstat is my hobby project written by rust. 


## How it works 

1. directly communicate with event device , the the userspace interface of input subsystem 
2. the architecture of the program
    - 捕获线程
    1. 针对每次事件，捕获数据 (time,key_type) 到 events 数组，
    2. 每隔一段时间，清空队列，并发送给显示线程

    - 显示线程
    1. 等待数据，做统计: 数据类型，数据总数，数据分布（A多少个）
    2. 显示数据，导出为直方图
```
   k_thread  m_thread    show_thread
   |______|_____________7
```

## Demo 

```shell

>>>> DEMO 

$ sudo charstop -k /dev/input/event5 -m /dev/input/event6 -top 4  5

## keyboard ## mouse ## 
## 123  ## 12 ##
> KEY_X ************
> KEY_A ***
> KEY_Y *******
> KEY_B ***

## 125  ## 16 ## 
> KEY_X ********
> KEY_A **************
> KEY_Y ***
> KEY_B ***


```



## Program Log
- [X] do capture
- [ ] implement IPC 
- [ ] move k_thread and m_thread as single capture_thread



