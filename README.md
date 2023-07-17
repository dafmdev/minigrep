# Bienvenido a mi proyecto personal en Rust

Este es un proyecto personal que estoy desarrollando como parte de mi aprendizaje de Rust. El proyecto está guiado por el libro de Rust, pero también he agregado algunas personalizaciones mías.

El proyecto es un grep simple que puede encontrar cadenas en archivos. Todavía está en desarrollo, pero ya puede hacer algunas cosas básicas.

Si está interesado en aprender Rust, o si simplemente quiere ver mi trabajo, ¡siéntase libre de echarle un vistazo!

## Cómo usar

Para usar el proyecto, simplemente ejecute el siguiente comando en la terminal:

```bash
cargo run -- <cadena> <archivo>
```

Por ejemplo, para buscar la cadena "hola" en el archivo "archivo.txt", ejecutaría el siguiente comando:

```bash
cargo run -- hola archivo.txt
```

Si quiere ignorar las mayúsculas, ejecutará el siguiente comando:

```bash
cargo run -- HoLa archivo.txt --ignore-case
```