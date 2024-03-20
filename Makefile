CREATOR_NAME =inz_creator
MKP_NAME =inz_mkp

build-cr:
	anchor build --program-name $(CREATOR_NAME)

deploy-cr:
	anchor deploy --program-name $(CREATOR_NAME)

build-mkp:
	anchor build --program-name $(MKP_NAME)

deploy-mkp:
	anchor deploy --program-name $(MKP_NAME)
