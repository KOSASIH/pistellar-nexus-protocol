variable "database_url" {
  type = string
  default = "postgres://user:password@host:port/dbname"
}

variable "stellar_network" {
  type = string
  default = "testnet"
}

variable "pi_network" {
  type = string
  default = "mainnet"
}
