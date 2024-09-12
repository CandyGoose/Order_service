$baseUrl = "http://localhost:3000/orders"

# Пример существующего идентификатора заказа для тестирования
$orderId = "b563feb7b2b84b6test"
$response = Invoke-RestMethod -Uri "$baseUrl/$orderId" -Method Get

# Пример несуществующего идентификатора заказа
$nonexistentOrderId = "nonexistentId"
$response = Invoke-RestMethod -Uri "$baseUrl/$nonexistentOrderId" -Method Get

# Тестирование обработки некорректного формата ID
$invalidFormatOrderId = "!@#$%&*<>"
$response = Invoke-RestMethod -Uri "$baseUrl/$invalidFormatOrderId" -Method Get
