## slmd - System Load Manager daemon

A small daemon for the **slm** application. It saves information about the CPU and RAM usage to a file.

-----------------------------
Description:

Настройки демона расположены в конфигурационном файле слмд.томл.
Описание настроек конфигурационного файла:
- out_file_path = "/tmp/slmd.out" - путь к файлу стандартного вывода
- err_file_path = "/tmp/slmd.err" - путь к файлу стандартного вывода ошибок
- data_file_path = "/tmp/slmd.dat" - путь к файлу сохраняемых данных
- working_dir = "/tmp" - рабочая директория демона
- root_dir = "/" - корневая директория демона
- user_name = "root" - пользователь-владелец демона
- group_name = "wheel" - группа пользователя-владельца демона
- tick_count = 3600 - количество сохраняемых записей в файле данных, после достижения указанного значения, файл обновляется
- tick_rate = 1000 - время между двумя сохраняемыми значениями, мс
-----------------------------

Operating systems supported:

  - Linux
  - MacOS

-----------------------------
### Version: 0.1.0
Dependencies:

- daemonize="0.5.0"
- csv="1.3.1"
- sysinfo = "0.33.0"
- confy = "0.6.1"
- serde = { version = "1.0.215", features = ["derive"] }
- chrono = "0.4.38"

