use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap"
        }
        
        main { class: "container",
            CV {}
        }
    }
}

#[component]
fn CV() -> Element {
    rsx! {
        section { class: "cv",
            // Шапка
            header { class: "cv-header",
                h1 { "Антон Гайдуков" }
                div { class: "contact-info",
                    span { "Москва" }
                    span { class: "separator", "•" }
                    a { href: "mailto:gaydukovus@gmail.com", "gaydukovus@gmail.com" }
                    span { class: "separator", "•" }
                    a { href: "tel:+79652439612", "+7 (965) 243-96-12" }
                    span { class: "separator", "•" }
                    a {
                        href: "https://t.me/akaBBMS",
                        class: "telegram-link",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M11.944 0A12 12 0 0 0 0 12a12 12 0 0 0 12 12 12 12 0 0 0 12-12A12 12 0 0 0 12 0a12 12 0 0 0-.056 0zm4.962 7.224c.1-.002.321.023.465.14a.506.506 0 0 1 .171.325c.016.093.036.306.02.472-.18 1.898-.962 6.502-1.36 8.627-.168.9-.499 1.201-.82 1.23-.696.065-1.225-.46-1.9-.902-1.056-.693-1.653-1.124-2.678-1.8-1.185-.78-.417-1.21.258-1.91.177-.184 3.247-2.977 3.307-3.23.007-.032.014-.15-.056-.212s-.174-.041-.249-.024c-.106.024-1.793 1.14-5.061 3.345-.48.33-.913.49-1.302.48-.428-.008-1.252-.241-1.865-.44-.752-.245-1.349-.374-1.297-.789.027-.216.325-.437.893-.663 3.498-1.524 5.83-2.529 6.998-3.014 3.332-1.386 4.025-1.627 4.476-1.635z"
                            }
                        }
                    }
                    span { class: "separator", "•" }
                    a { href: "https://github.com/akaAntonGaidukov", "GitHub: akaAntonGaidukov" }
                }
            }

            // Опыт работы
            section { class: "cv-section",
                h2 { class: "section-title", "Опыт работы" }

                div { class: "experience-item",
                    div { class: "experience-header",
                        h3 { class: "company-name", "metaLead"
                            span { class: "separator", "•" }
                            span { class: "position", "Аналитик (ML/CV)" }
                        }
                        span { class: "duration", "Июль 2024 — н.в." }
                    }
                    ul { class: "achievements",
                        li { "Разработал ML-систему скоринга клиентов: повышение конверсии с 1.9% до 3.6%" }
                        li { "Оптимизация времени обработки на 40%" }
                        li { "Динамическое распределение заказов (+4% успешных доставок)" }
                        li { "Стек: Python, Docker, REST API, Grafana" }
                    }
                }

                div { class: "experience-item",
                    div { class: "experience-header",
                        h3 { class: "company-name", "Aspentech"
                            span { class: "separator", "•" }
                            span { class: "position", "Техническая поддержка" }
                        }
                        span { class: "duration", "Июль 2022 — Декабрь 2022" }
                    }
                    ul { class: "achievements",
                        li { "Автоматизация ETL-процессов (Python-скрипты)" }
                        li { "Оптимизация SQL-запросов для геофизического ПО" }
                    }
                }
            }

            // Проекты
            section { class: "cv-section",
                h2 { class: "section-title", "Проекты" }

                div { class: "experience-item",
                    div { class: "experience-header",
                        h3 { class: "company-name", "Мониторинг загруженности курорта" }
                        a { 
                            class: "project-link",
                            href: "https://github.com/akaAntonGaidukov/stepanovo_trafic_pet",
                            "Исходный код"
                        }
                    }
                    ul { class: "achievements",
                        li { "Детекция объектов в реальном времени (YOLOv11 + OpenCV)" }
                        li { "Интеграция с FastAPI через WebSocket" }
                        li { "Визуализация метрик в Grafana" }
                    }
                }
            }

            // Образование
            section { class: "cv-section",
                h2 { class: "section-title", "Образование" }

                div { class: "education-item",
                    div { class: "education-header",
                        h3 { class: "company-name", "GeekBrains" }
                        span { class: "duration", "2022" }
                    }
                    ul { class: "achievements",
                        li { "Специализация 'Искусственный интеллект'" }
                    }
                }

                div { class: "education-item",
                    div { class: "education-header",
                        h3 { class: "company-name", "РГГРУ им. Орджоникидзе" }
                        span { class: "duration", "2015 - 2019" }
                    }
                    ul { class: "achievements",
                        li { "Геофизические методы поиска месторождений" }
                    }
                }
            }

            // Навыки
            section { class: "cv-section",
                h2 { class: "section-title", "Навыки" }

                div { class: "skills-container",
                    div { class: "skill-group",
                        h3 { class: "list-header", "ML/CV" }
                        ul { class: "skills-list",
                            li { "Python: PyTorch, OpenCV, YOLO" }
                            li { "Scikit-learn, Pandas, NumPy" }
                            li { "Docker, REST API, Grafana" }
                        }
                    }
                    div { class: "skill-group",
                        h3 { class: "list-header", "Базы данных" }
                        ul { class: "skills-list",
                            li { "SQL (MySQL, PostgreSQL)" }
                            li { "NoSQL (MongoDB, Redis)" }
                        }
                    }
                }
            }

            // Достижения
            section { class: "cv-section",
                h2 { class: "section-title", "Достижения" }
                ul { class: "achievements",
                    li { "Топ-10 на хакатоне LCT 2024 (детекция БПЛА)" }
                    li { "Снижение рисков при бурении на 30% через автоматизацию" }
                }
            }
        }
    }
}