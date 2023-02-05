
## Lógica de entidades:

question

form_question (fk form, question)

form

// Podría servir para conocer
// - Asignación de form a usuario
// - Intermedio con paper_answer para
//   determinar si form está completo
paper (fk user, form)

paper_answer (fk paper, answer)

answer (fk question)

---

Quizá un step vincula form con paper... a la vez que módulo u otros..

# TODO:
Para mañana el update... creo...
Quiero que me muestre la pregunta no solo el id
ummm revisar todo eso en paper
