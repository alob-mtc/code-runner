# code-runner
A remote code exec engin

The ideal is to have a peice of software that takes a set of code as imput and runs this in a docker container, return the result of execution as output 

```
input code [python, js]
          |
          |
          |
         \ /
Spin up a docker container from a pre-defined Docker-image
          |
          |
          |
         \ /
pass the code to the container engine
          |
          |
          |
         \ /
    execution output
          |
          |
          |
         \ /
    format outup 
```
