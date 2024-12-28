BASE_URL=:8080
FROM=2348a36a-8444-45a3-a130-669144a29793
TO=2348a36a-8444-45a3-a130-669144a29793

create-account:
	http POST $(BASE_URL)/accounts currency=1 uuid=$(FROM)

deposit:
	http POST $(BASE_URL)/deposits amount:=199.9 account_id=$(FROM)

withdraw:
	http POST $(BASE_URL)/withdrawals amount:=99.8 account_id=$(FROM)

transfer:
	http POST $(BASE_URL)/transfers amount:=99.8 from_account_id=$(FROM) to_account_id=$(TO)

balance:
	http $(BASE_URL)/accounts/$(FROM)