
## Lógica de entidades:

---

question

form_question (fk form, question)

form

paper (fk user, form) // -> Asignación form a usuario, para steps

paper_answer (fk paper, answer)

answer (fk question)

---

user // fuera de ámbito

---

Quizá un step vincula form con paper... a la vez que módulo u otros..

## Endpoinds

Documento [anexo](./README-endpoinds.md).

## Build process

Sería interesante automatizar el multi target para tener las imágenes con el mismo tag pero diferente platform. Notar que la imagen base es busybox

### Shared compilation

Permisos contenedor de compilación:

``` bash
sudo chmod -R o+w target
```

Compilar la aplicación:

``` bash
docker run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder

cargo build --release
```

Este proceso no cachea las dependencias por lo que realiza la compilación completa, [detalles](https://hub.docker.com/r/ekidd/rust-musl-builder/)

### Target busybox raspberry pi

Generar la imagen de docker.

``` bash
docker build --no-cache --pull --platform linux/amr64/v8 -f ./Dockerfile.arm -t kennycallado/uv-questions-api:v0-arm64 .
```

### Target busybox amd64

Generar la imagen de docker.

``` bash
docker build --no-cache --pull --platform linux/amd64 -f ./Dockerfile.amd -t kennycallado/uv-questions-api:v0-amd64 .
```

### Multi target

todo!()

## TODO:

- Crear imagen, subirla, desplegar en pi.

- [ ] q_type debería ser una tabla?
- [ ] Shadowing id ??

- [ ] Revisar lógica funciona correctamente.
- [ ] Unificar criterios en lógica.
  - [ ] Modelos
  - [ ] Repositorios
- [ ] Eliminar endpoinds sin uso
- [ ] Validación y autorización
- [ ] Realizar tests.

- [ ] CI/CD hacia hub.docker
