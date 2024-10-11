type Name struct {
	First string `json:"first"`
	Last  string `json:"last"`
}

type User struct {
	Name Name  `json:"name"`
	Age  *int  `json:"age"`
}