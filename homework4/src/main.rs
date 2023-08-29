pub struct MyVector<T> {
    data: Vec<T>,
}

impl<T> MyVector<T> {
    // Создать новый пустой вектор
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // Создать новый вектор с заданной емкостью
    pub fn with_capacity(capacity: usize) -> Self {
        Self { data: Vec::with_capacity(capacity) }
    }

    // Добавить элемент в конец вектора
    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    // Удалить последний элемент вектора и вернуть его
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    // Удалить элемент с определенным индексом и вернуть его
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index < self.data.len() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    // Получить ссылку на элемент с определенным индексом
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // Изменить размер вектора
    pub fn resize(&mut self, new_size: usize, value: T)
    where
        T: Clone,
    {
        self.data.resize(new_size, value);
    }
}

fn main() {
    // Создаем новый вектор
    let mut vec = MyVector::new();
    
    // Добавляем элементы
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    // Удаляем последний элемент
    if let Some(val) = vec.pop() {
        println!("Popped value: {}", val);
    }

    // Удаляем элемент с индексом 0
    if let Some(val) = vec.remove(0) {
        println!("Removed value: {}", val);
    }

    // Получаем и выводим элемент с индексом 0
    if let Some(val) = vec.get(0) {
        println!("Element at index 0: {}", val);
    }

    // Изменяем размер вектора
    vec.resize(5, 0);
    
    println!("Vector: {:?}", vec.data);
}