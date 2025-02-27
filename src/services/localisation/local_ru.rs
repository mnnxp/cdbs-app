use std::collections::BTreeMap;
use lazy_static::lazy_static;

lazy_static! {
    /// Translation of the fields for Russian
    pub(super) static ref LOCAL_RU: BTreeMap<usize, &'static str> = BTreeMap::from([
            (1, "инженер"),
            (2, "компонент"),
            (3, "поставщик"),
            (4, "Информация о 3D моделях, чертежах, стандартах и поставщиках"),
            (5, "Автоматизация доступа к платформе по API и интеграция с САПРами"),
            (6, "Публикация каталогов поставщиков и пользовательских компонентов"),
            (7, "Обмен информацией о компонентах, стандартах и производителях"),
            (8, "Английский"),
            (9, "Русский"),
            (10, "Соглашение и условия"),
            (11, "О нас"),
            (12, "Справочник к API"),
            (13, "Авторизация"),
            (14, "Создать аккаунт"),
            (15, "Профиль"),
            (16, "Настройки"),
            (17, "Выход"),
            (18, "Создать новый аккаунт?"),
            (19, "Имя пользователя"),
            (20, "Пароль"),
            (21, "Уже есть аккаунт?"),
            (22, "Эл.адрес"),
            (23, "Имя"),
            (24, "Фамилия"),
            (25, "Отчество"),
            (26, "Программа"),
            (27, "Регион"),
            (28, "создавая аккаунт, Вы соглашаетесь с "),
            (29, "положениями и условиями"),
            (30, "Обновлено: "),
            (31, "Закладчиков: "),
            (32, "СЕРТИФИКАТЫ"),
            (33, "ВСЕ КОМПОНЕНТЫ"),
            (34, "ИЗБ. КОМПОНЕНТЫ"),
            (35, "КОМПАНИИ"),
            (36, "ИЗБ. КОМПАНИИ"),
            (37, "ИЗБ. СТАНДАРТЫ"),
            (38, "ИЗБ. ПОЛЬЗОВАТЕЛИ"),
            (39, " Позиция: "),
            (40, " Регион: "),
            (41, " Рабочее средство: "),
            (42, "Скрыть информацию"),
            (43, "Раскрыть информацию"),
            (44, "Войти"),
            (45, "Создать"),
            (46, "Сохранить"),
            (47, "Удалить профиль"),
            (48, "Старый пароль"),
            (49, "Новый пароль"),
            (50, "Имя пользователя"),
            (51, "Тип компании"),
            (52, "Имя"),
            (53, "Фамилия"),
            (54, "Отчество"),
            (55, "Позиция"),
            (56, "Телефон"),
            (57, "Адрес"),
            (58, "Тип доступа"),
            (59, "Изменить"),
            (60, "Управление модификациями компонента"),
            (61, "Описание"),
            (62, "Введите пароль для подтверждения удаления профиля"),
            (63, "Профиль"),
            (64, "Сертификаты"),
            (65, "Доступ"),
            (66, "Сайт"),
            (67, "Удалить профиль"),
            (68, "Доступ обновлен"),
            (69, "Пароль обновлен"),
            (70, "Профиль удален"),
            (71, "будут удалены все связанные с профилем данные, без возможности восстановления!"),
            (72, "Обновленных полей: "),
            (73, "Последнее обновление: "),
            (74, "Сертификатов не найдено"),
            (75, "нет данных"),
            (76, "Открыть профиль"),
            (77, "Общая информация"),
            (78, "Аватар"),
            (79, "Выбрать поставщика"),
            (80, "Политика доступа"),
            (81, "ИНН"),
            (82, "Удаление профиля"),
            (83, "Загрузить сертификат"),
            (84, "Рекомендуется загружать сертификат в формате изображения."),
            (85, "Файлы, выбранные для загрузки"),
            (86, "Выберите файл сертификата"),
            (87, "Загрузить"),
            (88, "Очистить"),
            (89, "Успех"),
            (90, "Сертификат загружен!"),
            (91, "Рекомендуется загружать аватар в формате изображения."),
            (92, "Аватар обновлен!"),
            (93, "Выберите файл изображения"),
            (94, "загрузил "),
            (95, "обновлено "),
            (96, "Актуальное состояние"),
            (97, "Тип компонента"),
            (98, "Раскрыть"),
            (99, "Скрыть"),
            (100, "Модификации"),
            (101, "Характеристики компонента"),
            (102, "Файлы компонента"),
            (103, "Стандарты"),
            (104, "Каталоги"),
            (105, "Теги (ключевые слова)"),
            (106, "Файлы из набора файлов выбранной модификации"),
            (107, "Поставщики"),
            (108, "Основной поставщик"),
            (109, "Компания"),
            (110, "Наименование"),
            (111, "Действие"),
            (112, "Классификатор"),
            (113, "Допуск"),
            (114, "Тип доступа "),
            (115, "Вернуться"),
            (116, "Управление основными данными компонента"),
            (117, "Добавить"),
            (118, "Создано пользователем"),
            (119, "Файлы выбранной модификации"),
            (120, "Имя файла"),
            (121, "Тип"),
            (122, "Размер"),
            (123, "Добавление поставщика компонента"),
            (124, "Загрузил"),
            (125, "Дата загрузки"),
            (126, "Скачать"),
            (127, "Изменить"),
            (128, "Информация"),
            (129, "Выбрать"),
            (130, "Добавление параметра и его значения"),
            (131, "Добавление значения для параметра модификации"),
            (132, "Изменение значения параметра"),
            (133, "Установить значение"),
            (134, "Изменить значение"),
            (135, "Удалить"),
            (136, "Нет дополнительных параметров"),
            (137, "Получить ссылку"),
            (138, "Файлы из набора файлов для "),
            (139, "Сертификат удален!"),
            (140, "Описание обновлено!"),
            (141, "Владелец: "),
            (142, "Классификация: "),
            (143, "Открыть стандарт"),
            (144, "Допуск: "),
            (145, "тип доступа "),
            (146, "классификация"),
            (147, "допуск"),
            (148, "технический коммитет"),
            (149, "опубликовано"),
            (150, "состояние стандарта"),
            (151, "регион"),
            (152, "Характеристики стандарта"),
            (153, "Файлы стандарта"),
            (154, "Компоненты"),
            (155, "Дата публикации"),
            (156, "Обновлено"),
            (157, "Управление основными данными стандарта"),
            (158, "Поставщик"),
            (159, "Состояние"),
            (160, "произведен "),
            (161, "Открыть"),
            (162, "Лицензия"),
            (163, "Рег.№ "),
            (164, "Регион: "),
            (165, "Открыть компанию"),
            (166, "Добавить поставщика компонента"),
            (167, "Установить поставщика"),
            (168, "Выбрать поставщика"),
            (169, "Описание поставщика"),
            (170, "Наименование организации"),
            (171, "Сокращенное наименование"),
            (172, "Управление файлами модификации"),
            (173, "Управление наборами файлов модификации"),
            (174, "Добавить новую модификацию"),
            (175, "Создание новой модификации"),
            (176, "Наименование модификации"),
            (177, "Изменение модификации"),
            (178, "Параметр"),
            (179, "Значение"),
            (180, "Добавить параметр"),
            (181, "Добавление параметр для компонента"),
            (182, "Выберите изображение пред.просмотра"),
            (183, "Допустимые форматы"),
            (184, "Обновление изображения пред.просмотра"),
            (185, "Управление характеристиками компонента"),
            (186, "Загрузить файлы компонента"),
            (187, "Управление файлами компонента"),
            (188, "Файлы компонента"),
            (189, "Управление стандартами компонента"),
            (190, "Управление поставщиками компонента"),
            (191, "Добавить стандарт к компоненту"),
            (192, "Введите текст для поиска каталогов"),
            (193, "Введите ключевые слова (разделяя пробелом или запятой)"),
            (194, "Нет файлов для загрузки"),
            (195, "Выберите файлы для набора"),
            (196, "Добавить набор файлов"),
            (197, "Загрузить файлы в набор файлов"),
            (198, "Файлы из набора файлов"),
            (199, "Открыть компонент"),
            (200, "Выберите файлы для компонента"),
            (201, "Выберите файлы для модификации"),
            (202, "Загрузить файлы модификации"),
            (203, "Файлы модификации"),
            (204, "Файлов не найдено"),
            (205, "Задайте имя параметра (регистр букв имеет значение)"),
            (206, "Добавление набора файлов для выбранной модификации"),
            (207, "Выберите набор файлов"),
            (208, "базовый"),
            (209, "не базовый"),
            (210, "Наименование параметра"),
            (211, "Изменение значения параметра"),
            (212, "Выберите стандарт"),
            (213, "Данные обновлены! Кол-во изменений:"),
            (214, "Данные обновлены"),
            (215, "Измение представительства"),
            (216, "Тип представительства"),
            (217, "Удаление компонента"),
            (218, "Для подтверждения удаления всех данных компонента "),
            (219, ", введите его идентификатор:"),
            (220, "Подтвердить удаление"),
            (221, "Отмена"),
            (222, "Выберите файлы для стандарта"),
            (223, "Компания-владелец "),
            (224, "Управление характеристиками стандарта"),
            (225, "Файлы стандарта"),
            (226, "Открыть стандарт"),
            (227, "Удаление стандарта"),
            (228, " стандарта, введите его идентификатор:"),
            (229, "Указать характеристики стандарта"),
            (230, "Новое представительство"),
            (231, " регион "),
            (232, "адрес"),
            (233, "тип"),
            (234, "телефон"),
            (235, "тип представительства"),
            (236, "Имя файла"),
            (237, "Тип файла"),
            (238, "Размер файла"),
            (239, "Программа"),
            (240, "Загружен пользователем"),
            (241, "Дата загрузки"),
            (242, "Дата создания"),
            (243, "Ключевое слово должно быть менее 10 символов"),
            (244, "Добавление лицензии для компонента"),
            (245, "Выбрать лицензию"),
            (246, "Идентификатор:"),
            (247, "Путь:"),
            (248, "Условия использования CADBase"),
            (249, "Благодарим за пользование CADBase!"),
            (250, "Мы очень рады, что вы здесь. Пожалуйста, внимательно прочитайте это соглашение об условиях предоставления услуг, прежде чем получить доступ или использовать CADBase. Поскольку это такой важный договор между нами и нашими пользователями, мы должны сделать его как можно более понятным:"),
            (251, "ПРОГРАММНОЕ ОБЕСПЕЧЕНИЕ ПРЕДОСТАВЛЯЕТСЯ \"КАК ЕСТЬ\", БЕЗ КАКИХ-ЛИБО ГАРАНТИЙ, ЯВНЫХ ИЛИ ПОДРАЗУМЕВАЕМЫХ, ВКЛЮЧАЯ, НО НЕ ОГРАНИЧИВАЯСЬ ГАРАНТИЯМИ ТОВАРНОГО СОСТОЯНИЯ, ПРИГОДНОСТИ ДЛЯ КОНКРЕТНОЙ ЦЕЛИ И НЕНАРУШЕНИЯ ПРАВ. НИ ПРИ КАКИХ ОБСТОЯТЕЛЬСТВАХ АВТОРЫ ИЛИ ПРАВООБЛАДАТЕЛИ НЕ НЕСУТ ОТВЕТСТВЕННОСТИ ЗА ЛЮБЫЕ ПРЕТЕНЗИИ, УБЫТКИ ИЛИ ДРУГИЕ ОБЯЗАТЕЛЬСТВА, БУДЬ ТО ПО ДОГОВОРУ, В РЕЗУЛЬТАТЕ ПРАВОНАРУШЕНИЯ ИЛИ ИНЫМ ОБРАЗОМ, ВОЗНИКАЮЩИЕ ИЗ ПРОГРАММНОГО ОБЕСПЕЧЕНИЯ, ЕГО ИСПОЛЬЗОВАНИЯ ИЛИ ДРУГИХ ОПЕРАЦИЙ С НИМ, ИЛИ В СВЯЗИ С НИМ."),
            (252, "Пожалуйста, сообщите нам, если есть какие-либо ошибки или нужна помощь."),
            (253, "Связаться с нами: "),
            (254, "Сделано для лучшего настоящего"),
            (255, "CADBase — это платформа для обмена информацией о 3D-компонентах, чертежах и производителях. Вроде как GitHub для кода, только для компонентов (частей, метизов)."),
            (256, "Мы также рады приветствовать всех инженеров, архитекторов и просто хороших людей, которые любят делиться идеями, концепциями, опытом с другими. Вне зависимости от того, где вы живете, если вы хотите получать и делиться знаниями, то мы постараемся вам в этом помочь."),
            (257, "Платформа разрабатывается с 2018 года. При участии Ивана Носовского, Юлии Герасимовой и Ся Тяньхао (夏添豪)."),
            (258, "Мы надеемся, что вам понравится эта платформа."),
            (259, "При наличии пожеланий или намерений инвестировать в эту платформу, пожалуйста, сообщите нам."),
            (260, "Спасибо за использование CADBase!"),
            (261, "Показать профиль"),
            (262, "Имя файла: "),
            (263, "Добавление стандарта к компоненту"),
            (264, "Обновить компанию"),
            (265, "Открыть компанию"),
            (266, "Представительства"),
            (267, "Удаление компании"),
            (268, "Удалить компанию"),
            (269, "Уведомление о конфиденциальности"),
            (270, "Представительства компании не указаны"),
            (271, "Обновить доступ"),
            (272, "Внимание: "),
            (273, "это удалило все данные, связанные с компанией, это не может быть отменено!"),
            (274, "Удаление компании"),
            (275, " поставщик"),
            (276, "Создано"),
            (277, "Обновлено"),
            (278, " Эл.почта: "),
            (279, " Телефон: "),
            (280, " Рег.№ "),
            (281, " Местоположение: "),
            (282, " Сайт: "),
            (283, "Направления деятельности "),
            (284, "Уведомления"),
            (285, "Соглашение CADBase"),
            (286, "Члены"),
            (287, "Если вам нужна поддержка или помощь, пожалуйста, свяжитесь с нами: "),
            (288, "Замечательно!"),
            (289, "Создать компанию"),
            (290, "Создать компонент"),
            (291, "Создать стандарт"),
            (292, "Представительство компании удалено!"),
            (293, "Представительство компании создано!"),
            (294, "Дочерний компонент отсутствует"),
            (295, "Скрыть компоненты"),
            (296, "Показать компоненты"),
            (297, "Нет файла для отображения"),
            (298, "Развернуть"),
            (299, "Свернуть"),
            (300, "Просмотр"),
            (301, "Закрыть"),
            (302, "Оси координат"),
            (303, "Вращение"),
            (304, "Каркас"),
            (305, "Цвет модели"),
            (306, "Цвет фона"),
            (307, "Масштаб модели"),
            (308, "Ревизия"),
            (309, "Рев."),
            (310, "Размер"),
            (311, "Загрузил"),
            (312, "Создан"),
            (313, "Загружен"),
            (314, "Скрыть"),
            (315, "Показать"),
            (316, "байт(а)"),
            (317, "КБ"),
            (318, "МБ"),
            (319, "ГБ"),
            (320, "ТБ"),
            (321, "* при вводе данных обратите внимание на правильный порядок символов, регистр и язык ввода."),
            (322, "Копировать"),
            (323, "Скопировано"),
            (324, "Нет результатов"),
            (325, "Открыть 3D-вид"),
            (326, "Добавить в закладки"),
            (327, "Удалить из закладок"),
            (328, "Поделиться"),
            (329, "Показать файлы выбранной модификации"),
            (330, "Управление файлами стандарта"),
            (331, "Загрузить файлы для стандарта"),
            (332, "Ошибка авторизации"),
            (333, "Перейти на страницу авторизации"),
            (334, "Редактировать"),
            (335, "Предварительный просмотр"),
            (336, "Поддерживается язык разметки Markdown для создания форматированного текста."),
            (337, "Например, тег <sup></sup> используется для указания надстрочных символов, а строка «m<sup>2</sup>» преобразуется в запись квадратного метра."),
            (338, "Комментарий к изменению"),
            (339, "Комментарии помогут понять: почему сделано это изменение? какой эффект произвело изменение? для чего это было необходимо?"),
            (340, "Максимальная длина комментария 110 кириллических или 225 латинских символов (225 байт)"),
            (341, "Комментарий"),
            (342, "Импорт модификаций и параметров"),
            (343, "Чтобы загрузить данные, просто скопируйте их из электронной таблицы (Excel, LibreOffice и т. д.) и вставьте в поле ниже.<br/>Первая строка таблицы используется для указания наименований параметров.<br/>Для указания данных модификаций используются ключевые слова:<br/><b>[ModificationName]</b> - столбец с наименованиями модификаций (необходим),<br/><b>[ModificationDescription]</b> - столбец с описаниями,<br/><b>[ModificationActualStatusId]</b> - стобец для идентификаторов состояний."),
            (344, "Вставьте скопированные из таблицы данные. Пример:\n[ModificationName]\tпараметр 1\tпараметр 2\nимя модификации 1\tзначение 1\tзначение 2\nимя модификации 2\tзначение 3\tзначение 4"),
            (345, "заголовков"),
            (346, "строк"),
            (347, "Импорт"),
            (348, "Экспорт"),
            (349, "Поиск"),
            (350, "Нет результатов"),
            (351, "Введите текст для поиска"),
        ]);
}
