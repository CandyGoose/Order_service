#!/bin/bash

# URL вашего сервиса
BASE_URL="http://localhost:3000/orders"

# Пример существующего идентификатора заказа
ORDER_ID="b563feb7b2b84b6test"
echo "GET request for existing order ID:"
curl -i -X GET "$BASE_URL/$ORDER_ID"

# Пример несуществующего идентификатора заказа
NONEXISTENT_ORDER_ID="nonexistentId"
echo -e "\nGET request for non-existent order ID:"
curl -i -X GET "$BASE_URL/$NONEXISTENT_ORDER_ID"

# Тестирование обработки некорректного формата ID
INVALID_FORMAT_ORDER_ID="!@#$%&*<>"
echo -e "\nGET request with invalid order ID format:"
curl -i -X GET "$BASE_URL/$INVALID_FORMAT_ORDER_ID"
