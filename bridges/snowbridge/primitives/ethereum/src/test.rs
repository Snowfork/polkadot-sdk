// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2023 Snowfork <hello@snowfork.com>

use crate::{EIP1186Layout, StorageProof};
use ethereum_types::{H256, U256};
use hex_literal::hex;
use rlp::{Decodable, Rlp};
use rlp_derive::RlpDecodable;
use sp_io::hashing::keccak_256;
use sp_runtime::app_crypto::sp_core::KeccakHasher;
use trie_db::{Trie, TrieDBBuilder};

/// The ethereum account stored in the global state trie.
#[derive(RlpDecodable, Debug)]
struct Account {
	nonce: u64,
	balance: U256,
	storage_root: H256,
	code_hash: H256,
}

#[test]
fn test_can_verify_eip_1186_proofs() {
	// source?: https://medium.com/@chiqing/eip-1186-explained-the-standard-for-getting-account-proof-444bc1e12e03
	/*
	1. get the account proof for the gateway contract at block 20331823:
	curl https://eth-mainnet.alchemyapi.io/v2/cBKwkAM5EIpQr_fFSA6NuFEsVMI--fIZ \
	   -X POST \
	   -H "Content-Type: application/json" \
	   -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["0x27ca963c279c93801941e1eb8799c23f407d68e7",[],"0x1363D2F"],"id":1}' | jq .
	2. get the state root at block 20331823, https://etherscan.io/block/20331823
	 */
	let gateway_contract = keccak_256(&hex!("27ca963c279c93801941e1eb8799c23f407d68e7"));
	let proof = vec![
        hex!("f90211a02dbde9bf9fd637888f9a8816ac18f82afa489e3ab1ffc91b707007817400fa0ca002f0617e39707a966f4099a617551727a4ba1aaea36014d2344240ff6c8a9e64a003d525e8569b49af0d87fd3d1831fe30dff825089f309c813b4ff7e32e873d81a00b3247fdbc6b46742ba157af6da00273ef25e19d6c5fed64e335d3e1f2580eb2a0549947a85b1ce8a738203963c541fc01116fd2d34bb2f8867bd6b8bf839221b7a0ab61757804b0bbd3fb1da1d79ef9282bb16d9be6f1f2c329a7b60f53dd65c791a01b2cb1d132246536227bd46188190552e0dffd629ea86eb3a6786e9433912c06a035eeec397ced133abd756142a88cb2681ee6bc6f95122a0f0423313665ade408a094dad584d8c7f44065679748a44223d59bd741f5fa34cc0177a6201f317fc59fa04b577832448242dd545a17edd8d874cfb468507ed793ac5a858a99a87c108d11a02a44d1ab98dfee7b65788fe26c304159f3eef92c8e77632d696b374b427ff85fa0a11f5b46b24c2840091897b0b8c6cc70e0f133351e3f46e5e4d5c764df451d06a0416bfebc9f3a4c405e7daac063e0837bddc1c11b74b6cc8f86829512eb49652aa0685c21643bb29c073476312f292ffae1d6d408347a59bdf9efc7d700248d2765a07cad2da479f20ecc028dff6356bd8b9046ca5fdc0cb371e0b3e0a4aea191a400a059651a44377b27534e2aa9b81d4694cdf1e80703a7061786cb4499d91818167580").to_vec(),
        hex!("f90211a0775e02b95ea6d3ef7cfcee269794a3fb01915d8fe94dd044a4ab0c8eba551aeaa02d166e28cc127a8674a61b6dec39f3fb8c0f13d4de7c2d07a6d1be2490bdfb87a0ef9e3ac272000f42fa38f4222a484c32bd18d44d8e3fc34434fff7a484e97247a01999972ffa874faf5a77f1c97e699513c4c2283791cf6d9459636535963f4179a05eb4b9e07252432597220c57222b7473571c3f69b3e77d4cdd2fd6de247929aba05b1270e2c35b77bb659d2932327c859b3f5b3e61cfcf7f8baf4ffe909b276130a05f58749ffbabdc6ec437db8fd0e91ba98f9f69daaea819f775a4c0d86ff3df79a0c99228a94352613de560b69f962ab09eab42d27c5cbf67fd86268d972feb54b2a06758723bfdc089d59b62513da213f277cb651182290ec12bb7fea80ea20bbf71a0eb7f70e3f367a00354447e0fa0744c325da54cf58f1014bfca4020a85153403ca0ab4cd31b874a352a86219ac1a5b9236e8e32c6833ea48683d9306ea4fbc6f2bda06a437082e4476a4f140a3f79c23ce27913f5e061e26559e134d045db64c9c186a0e8c869cbe0739b59b80ca3f9fc0028a608b29e3e97244e01a29b22c596ecc135a0115b82aaa306335ee5fbfc5a88f9dff9a0a545ceec6f0c19268d1f4d4e978daca02ffe21043e493282d53c128fc3b0cb7c715c13a543303839d3975790c66cc9a5a0bad0d2a614f382cea6060bf05025d865216df8bc6ca08bd4be5a99cff3d0b3cc80").to_vec(),
		hex!("f90211a0dde4c765a70ca6769b0eb2a0bf3107a0dd994160e71dced600689e113c11fa1ea06337854d2fd29d979e6f9c08b86cbb03b6f87a84a4bf6c858a0b78667e6984cfa0e069656281aa908dbf70b5234a8f1f35a5c800b7846ed92f378baaaa849735e3a03a25e49731b8eec23469f9ec6a043c4a70566cb3bb14a42889ccf7b1a60c9c58a09350958e1ced9996aa1799a5a7bbecb983722cbd54cafb171f963875910da180a0a24c484455900e512be7c3a1b14d18217fd55122a369dba4525908e264c88026a0631712a24201859596808c89ade5694ad38b961f92b518fe91799b4b17541a3fa038884b7b196e70b6282de77584cdf64ee57aa60ee23196f1867acab5581ad51da0a41b6c980513501689fb64aa5d96adc9a62d4b6baffe3d18c23351c3d837c502a0cf9a6b30d8f3caba30d8bd9c0edf3c6214ec8ee314b40843014a3ffb6cead30da021a63a8e7f2dba67b93097344f769446f9b33284975db30c4b66dfb9787b92d7a0f82206f757bf84bf7baa8a3457761ccea7fd0572e90c325f66ecc85c9b5a6f46a0b6e2c101153c997455dd863bfcbf52ec344b4aee8b182709d1189831c3f12d20a0285fb759f5c37fcb814a372ccd6f19b2b2b29556d0b613ff44951f6a46038f89a0d103030270fd49a9dc0f36c038eb9b1b3e3f5545d34535cb951bfd00022b027fa04637abdd91cd85668dab010d780ae51f3ff3b637b46038e397c982161df47f7480").to_vec(),
        hex!("f90211a00cfd6859835505f61eb0922926e7c84571853c49f7073c880aed9d6d14edc35ba07c62828b0512ae20010e54f7f4d7373e46e5d684763d38ee312957c9d8221772a0c45e4298476c02f63193a5da673a4c68bc75ba66e87390ad17ff8c239f29f9fea034625f11497d473938a198d402f5ea67fbabacf150730f4c9487c71bff3f6f28a0e68c9bdf5be655aab0dec22da0538325d0b291c1a4a15328905fbe7bc0348e9da08096a49e9e41d5db639cf1a4fbc610e0841dd210eeb998a7d03ae45020cf730ba0dcd2fe58afa8cf27d3d022a0383b23f9412c84cc9da708142b2939e9a9b07217a0550607e44be5d76e5f881542f7d66ebd2ad77c5f2625329f8c6ad6c4120b832ca083a53c9cf49e5a6d8f351fc11c477a1e1048ad860c2bfdc5cc840e0ec704b8cba0c56b465f93689fc88b5a2b191b64fb134a290bfcce1c2c2ecb32f73786be22eaa0583ea27e9300f98239bd79a3b1d79efc1c94f65b98640bf45ce459e06e2bb15da0087b82f58e946e48fccfd5cfe9d067dd5c09bffe4db6071318a099ce5b5cced1a0868f6e53feff9bcf6537493d9c109c4daf9b1fb7f9a1c1da7899e3940ad1e506a0bc8f3ff56f10135a0f00abc8d5c9f0f1387bb4b5df6047d095316423586fcda2a0156483204aa11b3b026976ce2531a74a5f322e4b89907cabea0f3d2242400769a01abb7f2e2dc4377284d4dd2d1707bec870c8be4c60634bf14ca70522e688085d80").to_vec(),
        hex!("f90211a00814bc7cfaa613b371f9a0b0d2cbfb86dd90c7b09d9bf6b876035ec1ae925cb7a06e5f65b114052f3cbe2dd38e354a618b8644241278e27fb50dda92472cf2f2eaa00275ca4b3f2ccb64b58cb346e500353cde0873d000fdb6ba32d0cd38100d66afa0572e378be18ace489ef903a1bbfd506fea558ef15c1c18df8d8ce0f879637da1a06f31c506e4e05e167bb6d59490768fc8446413e9253be0b603f164ea3923538aa071e16b0a3d39eb20f5d5979e6c95c0b0d0809a62c462fb9c4353529d0b824c76a0237ccc194553f7d9eacbe96143217f47d2267064cb9d37486ce1889de35edd41a052c303d389a9b14918d41a49bc72608fc7636fb1063d472e1ca3a8e9ead3480ba0cfcf66184f840b73f33c3e8ed1d84b441df402da3df2a0c1dcaa49b335cf1e1ea0461b09e9b8aa3a9b0c3d66bd559b249e1347e77cd26f696448a9cd6f3298f567a03df53a24722aac2e07c345177feff648f27bce658aba06ab0d323cf1bc44ca4ca01e4fc26f88ddd8637bcbed80403a54cbc235696d79ed7b0c076d1c92569c5074a0290ef3f1933759bc5af46a971ef8de9a8a546d95746c2413fe9b4d9ae60e6fa0a0bb3c7fa400fdec860edf62b93b35e15cbd33eacdbc082de96ec98b4238d7f429a0a8caff272cb20bee3785191f7d100d272b8fbcc85e3f3b12db898762ad88dcc6a0dde002fedc8e0f087fe7ac12048026defe2b83783ff948b4ec8adcef254604d980").to_vec(),
        hex!("f90211a0b77c2dcbc8e1db4884b4429475f64e46298aa58fbd5e9f70def2c47d26dd5608a049e72effaef78cce202e384cca13f4042687f07647dba61c71077fcfa126b318a03349bdb85883cd06f27ad73390733edc21efbfde6f2e6efd318b90cbe1f475afa00b253f431feb8392bd8082bde8c98b115b0437299f80086b28797b16c8ef6afaa0599b1fc9828953d465f16e4c3cff72b70558ca28c2f726a25bf04c9e0fe7c500a0162e095290db2c4aa447b94efa839493fe8b99c02366f78a4818463db7b486faa0674ee532c1f1ce5ef8b71c72b9e8d9ed098b09a93572367984e63d79bc0e1518a004873f382b064402dbbaf5e358a6476b5e1dac3c330136944b66559a21bd470ea05ea2e21a4205c8d8c66b6856950569626ea11abfc78351e0faf6e8e16c7f7045a08a09f9a3a9378a75750fc7ac8f1b5b471e8fa9b7e2ae4a1aebc4fb31f84d6a59a0d5588977f682d1b9d2a08b0d7513e5a52b1011b94cc8e6f8fdd416bfd6ebdb3da06a007da877569cde0e8dd3e3444eb4fa38ec44f774f3c035f1ffe76685fc602ea0b38f60ddfea5e9f833248c0691defed1eb9c00779fee9ab3c63e0c49b218037fa070262edcdd2c251b56dba079dc77fa058de2560976250f66383f724d4358767ca047d2990c0ac79e9ab2cb1509bb2a8e8e354ce10e151017b2db32aa3f08fb3c7fa0de47fd36220e45c89f43f07700e96c1303c40563532602c0518fec8877787c3d80").to_vec(),
        hex!("f901b1a0af5f5b69d829638cd348889559d7e1384570706f89f61d0376a15bbde1a37a6c80a02d3e337715ce19fc4dbedfe6c7aa078e4f3fef102f1e1bb033f83215b01a13caa0f423efcb0121d9659d7b4c49e917029c2e1501c17f719482757089ef87b7903f80a0135c09510565cfe238ee6b43a691771a311e92dcd9c516b71d4fcf3fc3f7c456a08eb40f8fa37e7a17ddc2fb80af5a36cb9c8b599daf05f20acd7182e6728653b7a087e2c4304c0e561f58ed597c5b71d380af9dae964efc3a6d22ce49d96af77e32a06e1dc105ed9a37862f59a4bcb95aeaee3fcbef48457ff4cce517b093834f275fa09f343d4d8ab40b83d3de9a42fffbad0a26c0a415948fc6825e134dcb6a75118da0283c358e0eeddac96746dbedb6114d38d906dda3c4ec36ddfd611c12eb26db75a0f2b5f06fbe770610f727b3fb7d60660c3459b203b68b91c5f68a99ec4e69ddad80a083a342a91d1ed5286711585a724a7d88309cff5508cf9fff902ad35517e07b43a0277561f92f2ec29ab6f27af2cb607ee9068b6be6c1f880618f820ed33849f6f8a0f7fe19e8f5ed3c6c75629cc9b609017f7c05affd0322d47db2ba30d6e678849280").to_vec(),
        hex!("f8718080808080808080a0a33e2cbef0b9a32b10c5a4e9fb38548571b7e5bf86f1ca262e947da048e2443980808080a0d1d86a672be9099ae789e91df45e54eeec5a49a178eacaee160d9fe84355883ba0e4d1c3c700d63192e7248f7f438bdf38442eabc0100adde67b0e2eccf978a4c48080").to_vec(),
		hex!("f8669d20a7e0fe655df4051644711bc6f37481a29575edb4ec4f1dbe3803bdb5b846f8440380a0e2914d03b741b7131b00e88381793e2d93b33d9c546f7f78f8f83fefb3395307a0a64a5bd2f4403a9590cb4ffaabc86faf90337177ff073ad5a38d9e36b8116d25").to_vec(),
    ];
	//
	let root = H256(hex!("87f90c94904e5328bf07bafe083476a20b5398e7295136dd649bbdf7f45b33ed"));

	let db = StorageProof::new(proof).into_memory_db::<KeccakHasher>();
	let trie = TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&db, &root).build();
	let result = trie.get(&gateway_contract).unwrap().unwrap();

	// the raw account data stored in the state proof:
	let account = Account::decode(&Rlp::new(&result)).unwrap();

	// some assertions about the account data in the state root.
	assert_eq!(
		account.storage_root,
		H256::from(hex!("e2914d03b741b7131b00e88381793e2d93b33d9c546f7f78f8f83fefb3395307"))
	);
	assert_eq!(account.nonce, 0x03);

	/*
	1. For Slot CoreStorage.channel, the key of AssetHub is:
	keccak256(c173fac324158e77fb5840738a1a541f633cbec8884c6a601c567d2b376a0539 ++
	(keccak256("org.snowbridge.storage.core") + 1)) =
	91839d9989408fbab863f2059ae80fee5216f58ec04fa3bffb021275bf7d4f23
	2. get the storage proof for the key at block 20331823:
	curl https://eth-mainnet.alchemyapi.io/v2/cBKwkAM5EIpQr_fFSA6NuFEsVMI--fIZ \
	   -X POST \
	   -H "Content-Type: application/json" \
	   -d '{"jsonrpc":"2.0","method":"eth_getProof","params":["
		 0x27ca963c279c93801941e1eb8799c23f407d68e7",["
		 0x91839d9989408fbab863f2059ae80fee5216f58ec04fa3bffb021275bf7d4f23"],"0x1363D2F"],"id":1}' | jq .
	/// @dev A messaging channel for a Polkadot parachain
	struct Channel {
		/// @dev The operating mode for this channel. Can be used to
		/// disable messaging on a per-channel basis.
		OperatingMode mode;
		/// @dev The current nonce for the inbound lane
		uint64 inboundNonce;
		/// @dev The current node for the outbound lane
		uint64 outboundNonce;
		/// @dev The address of the agent of the parachain owning this channel
		address agent;
	}
	*/
	let key = hex!("91839d9989408fbab863f2059ae80fee5216f58ec04fa3bffb021275bf7d4f23");
	let storage_proof = vec![
		hex!("f90191a0b8e9c10d7c4c95f0eedfd523680165eed24fa084bd3ac0c8659fc6d3f958116aa03aa5e2b212e3475262966022f3ffb5852ce7b366665b8a5987f79fbdb883f2afa0397a78b0cfa2d4b0092720e3619d9be0dd5ef3866bd2d180eb1757027aa3266da0f4d837dd20bb4f08979738df3c691aa62c16d7d1ae9515db1e6a8da4da409a5ea0c3c0f253a13a381796b8bc4fba463531f8e8d0ee8ddcd0b075c424dd4348f4908080a082877502399ec86295eba12860af86472063ad63d55a5935d8be8a3d309277a580a0012fff2a739f64fab984cb658df77a3ab76c6b7efda95b5b9a7d5069ec7faa94a0e0c4897bfecca6b1c87cfe856a914ec2ab043525368eabbf7b85216d9d0b5640a0d280b708d936d5fd907604e063b54581348e8a17631a5ae7c1c3d369a7edeb61a0e8e7a3641c070b352f900864923c204a9c22b20a8123eb948a34037039bb03ada0dbe8d231dd5126e78b689422047f3b667466e9a0584a095468cf3a2ee907990380a0e085b6c5b2b4cf22c1ccb0cc32eeb19fd278c7324149296a58643e67226cfc9780").to_vec(),
        hex!("eda03dd682974e695f0b903d02ea04e5835154d6810113c23898412593f6b30379a58b8a5e000000000000000800").to_vec(),
	];
	let proof_db = StorageProof::new(storage_proof).into_memory_db::<KeccakHasher>();
	let trie =
		TrieDBBuilder::<EIP1186Layout<KeccakHasher>>::new(&proof_db, &account.storage_root).build();
	let _result = trie.get(&key);
	println!("hello");
}
