cargo make sign

sc.exe create example binPath= "C:\Users\Lenovo User\CLionProjects\driver\target\x86_64-pc-windows-msvc\debug\driver.sys" type= kernel

sc start example
sc stop example
sc delete example
