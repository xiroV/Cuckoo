# Cuckoo Mail Client

## Configuration
Config file should be stored in `/home/<user>/.config/cuckoo/config.toml`

Currently only supports one or more accounts, as following:

```
[accounts]
	[accounts.personal]
	name = "John"
	mail = "example@example.com"
	imap_server = "mail.example.com"
	imap_user = "johndoe"
	
	[accounts.work]
	name = "John Doe"
	mail = "example@work.com"
	imap_server = "work.example.com"
	imap_user = "jd"

	...
```

