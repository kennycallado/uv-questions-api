
- >> (index) GET /health
  - String -> \"questions api ready\"

- **Question**
  - >> (index) GET /api/v1/question/

    - Devuelve -> Vec\<Question\>
    ``` json
    [
      {
        "id": 1,
        "q_type": "range",
        "question": "¿Bla bla bla bla bla bla?"
      },
      ...
    ]
    ```

  - >> (store) POST /api/v1/question/
    - Recibe -> NewQuestion
    ``` json
    {
      "q_type" : "range",
      "question": "What is the answer to life, the universe and everything?"
    }
    ```

    - Devuelve -> Question
    ``` json
    {
      "id": 6,
      "q_type": "range",
      "question": "What is the answer to life, the universe and everything?"
    }
    ```

  - >> (show) GET /api/v1/question/\<id\>
    - Devuelve -> Question
    ``` json
    {
      "id": 6,
      "q_type": "range",
      "question": "What is the answer to life, the universe and everything?"
    }
    ```

  - >> (destroy) DELETE /api/v1/question/\<id\>
    - Devuelve -> Question
    ``` json
    {
      "id": 6,
      "q_type": "range",
      "question": "What is the answer to life, the universe and everything?"
    }
    ```

  - >> (update) PUT /api/v1/question/\<id\>
    - Devuelve -> Question
    ``` json
    {
      "id": 6,
      "q_type": "range",
      "question": "What is the answer to life, the universe and everything?"
    }
    ```

- *Form*
  - >> (index) GET /api/v1/form/
    - Devuelve -> Vec\<FormWithQuestions\>
    ``` json
    [
      {
        "id": 1,
        "title": "titulo 1",
        "description": "Questionnaire 1 of 9",
        "questions": [
          {
            "id": 1,
            "q_type": "range",
            "question": "¿Bla bla bla bla bla bla?"
          },
          ...
        ]
      },
      ...
    ]
    ```

  - >> (store) POST /api/v1/form/
    - Recibe -> NewForm
    ``` json
    {
      "title": "titulo 10",
      "description": "Questionnaire 10"
    }
    ```

    - Devuelve -> Form
    ``` json
    {
      "id": 10,
      "title": "titulo 10",
      "description": "Questionnaire 10"
    }
    ```

  - >> (show) GET /api/v1/form/\<id\>
    - Devuelve -> FormWithQuestions
    ``` json
    {
      "id": 1,
      "title": "titulo 1",
      "description": "Questionnaire 1 of 9",
      "questions": [
        {
          "id": 1,
          "q_type": "range",
          "question": "¿Bla bla bla bla bla bla?"
        },
        ...
      ]
    }
    ```

  - >> (destroy) DELETE /api/v1/form/\<id\>
    - Devuelve -> Form
    ``` json
    {
      "id": 10,
      "title": "titulo 1",
      "description": "Questionnaire 1 of 9"
    }
    ```

  - >> (update) PUT /api/v1/form/\<id\>
    - Recibe -> FormWithQuestions
    ``` json
    {
      "id": 11212,
      "title": "titulo bla bla",
      "description": "Questionnaire 1 of 9",
      "questions": [
        {
          "id": 4,
          "q_type": "patata",
          "question": "patatas everywhere"
        },
        ...
      ]
    }
    ```

    - Devuelve -> FormWithQuestions
    ``` json
    {
      "id": 11212,
      "title": "titulo bla bla",
      "description": "Questionnaire 1 of 9",
      "questions": [
        {
          "id": 4,
          "q_type": "range",
          "question": "¿Bla bla bla?"
        }
        ...
      ]
    }
    ```

- *Paper*
  - >> (index) GET /api/v1/paper/
    - Devuelve -> Vec\<PaperWithAnswers\>
    ``` json
    [
      {
        "id": 1,
        "user": {
          "id": 1,
          "email": "bla@asflkjd.es",
          "token": "1234567890"
        },
        "form": {
          "id": 1,
          "title": "titulo 1",
          "description": "Questionnaire 1 of 9"
        },
        "answers": [
          {
            "id": 1,
            "question": {
              "id": 1,
              "q_type": "range",
              "question": "¿Bla bla bla bla bla bla?"
            },
            "answer": "A"
          },
          ...
        ]
      },
      ...
    ]
    ```

  - >> (store) POST /api/v1/paper/
    - Recibe -> NewPaper
    ``` json
    {
      "user_id": 2,
      "form_id": 4
    }
    ```

    - Devuelve -> PaperWithAnswers
    ``` json
    {
      "id": 10,
      "user": {
        "id": 2,
        "email": "lsdfj@pwoei.es",
        "token": "1iasd0f929"
      },
      "form": {
        "id": 4,
        "title": "titulo 4",
        "description": "Questionnaire 4 of 9"
      },
      "answers": []
    }
    ```

  - >> (show) GET /api/v1/paper/\<id\>
    - Devuelve -> PaperWithAnswers
    ``` json
    {
      "id": 2,
      "user": {
        "id": 1,
        "email": "bla@asflkjd.es",
        "token": "1234567890"
      },
      "form": {
        "id": 2,
        "title": "titulo 2",
        "description": "Questionnaire 2 of 9"
      },
      "answers": [
        {
          "id": 4,
          "question": {
            "id": 1,
            "q_type": "range",
            "question": "¿Bla bla bla bla bla bla?"
          },
          "answer": "D"
        },
        ...
      ]
    }
    ```

  - >> (destroy) DELETE /api/v1/paper/\<id\>
    - Devuelve -> PaperWithAnswers
    ``` json
    {
      "id": 10,
      "user": {
        "id": 2,
        "email": "lsdfj@pwoei.es",
        "token": "1iasd0f929"
      },
      "form": {
        "id": 4,
        "title": "titulo 4",
        "description": "Questionnaire 4 of 9"
      },
      "answers": [
        ...
      ]
    }
    ```

  - >> (update) PUT /api/v1/paper/\<id\>
    - Recibe -> NewPaperWithData
    ``` json
    {
      "id": 10,
      "user_id": 2,
      "form_id": 4,
      "answers": [
        {
          "question_id": 1,
          "answer": "BLA"
        },
        {
          "question_id": 2,
          "answer": "B"
        }
      ]
    }
    ```

    - Devuelve -> PaperWithAnswers
    ``` json
    {
      "id": 9,
      "user": {
        "id": 2,
        "email": "lsdfj@pwoei.es",
        "token": "1iasd0f929"
      },
      "form": {
        "id": 4,
        "title": "titulo 4",
        "description": "Questionnaire 4 of 9"
      },
      "answers": [
        {
          "id": 13,
          "question": {
            "id": 1,
            "q_type": "range",
            "question": "¿Bla bla bla bla bla bla?"
          },
          "answer": "BLA"
        },
        ...
      ]
    }
    ```

### Deprecables

Los siguientes endpoinds pueden ser eliminados ??

- *Answer*

  - >> (index) GET /api/v1/answer/
    - Devuelve -> AnswerWithQuestion
    ``` json
    [
      {
        "id": 1,
        "question": {
          "id": 1,
          "q_type": "range",
          "question": "¿Bla bla bla bla bla bla?"
        },
        "answer": "A"
      },
      ...
    ]
    ```

  - >> (store) POST /api/v1/answer/
    - Recibe -> NewAnswer
    ``` json
    {
        "question_id": 1,
        "answer": "BLA"
    }
    ```

    - Devuelve -> AnswerWithQuestion
    ``` json
    {
      "id": 15,
      "question": {
        "id": 1,
        "q_type": "range",
        "question": "¿Bla bla bla bla bla bla?"
      },
      "answer": "BLA"
    }
    ```

  - >> (show) GET /api/v1/answer/\<id\>
    - Devuelve -> AnswerWithQuestion
    ``` json
    {
      "id": 5,
      "question": {
        "id": 2,
        "q_type": "range",
        "question": "¿Bla bla bla bla bla?"
      },
      "answer": "E"
    }
    ```

  - >> (destroy) DELETE /api/v1/answer/\<id\>
    - Devuelve ->  AnswerWithQuestion
    ``` json
    {
      "id": 15,
      "question_id": 1,
      "answer": "BLA"
    }
    ```

  - >> (update) PUT /api/v1/answer/\<id\>
    - Recibe -> NewAnswer
    ``` json
    {
      "question_id": 5,
      "answer": "BLA"
    }
    ```

    - Devuelve -> AnswerWithQuestion
    ``` json
    {
      "id": 5,
      "question": {
        "id": 5,
        "q_type": "range",
        "question": "¿Bla bla?"
      },
      "answer": "BLA"
    }
    ```

- *User*
  - >> (index) GET /api/v1/user/
    - Devuelve -> Vec\<User\>
    ``` json
    [
      {
        "id": 1,
        "email": "bla@asflkjd.es",
        "token": "1234567890"
      },
      ...
    ]
    ```

  - >> (show) GET /api/v1/user/\<id\>
    - Devuelve -> User
    ``` json
    {
      "id": 1,
      "email": "bla@asflkjd.es",
      "token": "1234567890"
    }
    ```
  
  - >> (update) PUT /api/v1/user/\<id\>
    - Recibe -> 
    ``` json
    {
      "id": 1,
      "email": "kenny@blabla.es",
      "token": "092834342890fkojlaskfd"
    }
    ```

    - Devuelve ->
    ``` json
    {
      "id": 1,
      "email": "kenny@blabla.es",
      "token": "092834342890fkojlaskfd"
    }
    ```
