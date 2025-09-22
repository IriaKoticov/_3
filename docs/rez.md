# Петрозаводский Государственный Университет
**Кафедра Физики Твердого Тела**

## Лабораторная работа по информатике
### Тема: Генерация случайных данных и запись в файлы

**Выполнил:** Науменко И.М.
**Проверил:** Лобов Д.В.

---

### Цель:
Написать программу, которая генерирует случайные числа и записывает их в файлы, имена которых указаны в конфигурационном файле.

### Задача:
- Для каждого имени файла сгенерировать массив из 10 случайных целых чисел от 1 до 10.
- Записать сгенерированные числа в соответствующий файл.

### Идея метода:
1. Чтение строк из файла `config` в вектор строк.
2. Для каждой строки (имени файла) создается файл и заполняется случайными числами.
3. Случайные числа генерируются с помощью функции `rand()`, приведенной к диапазону 1–10.

### Алгоритм (псевдокод):
```Начало

    Прочитать файл "config" в вектор name.

    Для каждого имени файла в векторе name:
    2.1 Открыть файл для записи.
    2.2 Сгенерировать вектор из 10 случайных чисел.
    2.3 Записать числа в файл через пробел.
    2.4 Закрыть файл.
    Конец
```

### Листинг программы (C++):

```cpp
#include <cstdlib>
#include <iostream>
#include <fstream>
#include <vector>

std::vector<std::string> get_name(std::string path){
    std::string line;
    std::ifstream in(path);
    std::vector<std::string> confignamebox;
    if (in.is_open())
    {
        while (std::getline(in, line))
        {
            confignamebox.push_back(line);
        }
    }
    in.close();
    return confignamebox;
};

std::vector<int> boxwithrandom() {
    std::vector<int> tmp;
    for (int i = 0; i < 10; i++) {
        tmp.push_back(1 + rand() % 10);
    }
    return tmp;
}

int main(){
    std::vector<std::string> name = get_name("config");
    for (std::string i : name){
        std::ofstream fd(i);
        if (fd.is_open()){
            for (int num : boxwithrandom()){
                fd << num << " ";
            }
            fd << std::endl;
        }
        fd.close();
    }
    return 0;
}```
