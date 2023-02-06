
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

## TODO:

- q_type debería ser una tabla?
- Shadowing id ??

- [ ] Revisar lógica funciona correctamente.
- [ ] Unificar criterios en lógica.
  - [ ] Modelos
  - [ ] Repositorios
- [ ] Eliminar endpoinds sin uso
- [ ] Validación y autorización
- [ ] Realizar tests.

