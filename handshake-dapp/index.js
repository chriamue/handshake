let getPolkadotJsExtensionMod = (() => {
  let mod = null;

  let initPromise = (async () => {
    mod = await import(
      "https://cdn.jsdelivr.net/npm/@polkadot/extension-dapp@0.46.5/+esm"
    );
  })();

  return async () => {
    if (mod == null) {
      await initPromise;
    }
    return mod;
  };
})();

// Load the contract metadata from a JSON file
async function loadContractMetadata() {
  const response = await fetch("res/handshake.json");

  if (!response.ok) {
    throw new Error("Failed to fetch the contract metadata");
  }

  return await response.json();
}
async function initApi() {
  const { ApiPromise, WsProvider } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api@10.9.1/+esm"
  );

  // Check if the app is running on localhost
  const isLocalhost = ["localhost", "127.0.0.1"].includes(window.location.hostname);

  // Use localhost if in development, else use the testnet provider
  const providerUrl = isLocalhost ? "ws://127.0.0.1:9944" : "wss://ws.test.azero.dev";

  const provider = new WsProvider(providerUrl);
  const api = await ApiPromise.create({ provider });

  return { api };
}

async function getAccounts() {
  const extensionMod = await getPolkadotJsExtensionMod();
  await extensionMod.web3Enable("Handshake App");
  const allAccounts = await extensionMod.web3Accounts();
  const accountObjects = allAccounts.map((account) => ({
    name: account.meta.name,
    source: account.meta.source,
    ty: account.type,
    address: account.address,
  }));
  console.log(accountObjects);
  return JSON.stringify(accountObjects);
}


async function queryContract(contractAddress, queryFunction, ...args) {
  const { ContractPromise } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api-contract@10.9.1/+esm"
  );

  const { api } = await initApi();

  const metadata = await loadContractMetadata();
  const contract = new ContractPromise(api, metadata, contractAddress);

  const { BN, BN_ONE } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/util@12.4.1/+esm"
  );

  const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
  const PROOFSIZE = new BN(1_000_000);

  const gasLimit = api?.registry.createType("WeightV2", {
    refTime: MAX_CALL_WEIGHT,
    proofSize: PROOFSIZE,
  });

  const storageDepositLimit = null;

  const { gasRequired, storageDeposit, result, output } = await contract.query[queryFunction](
    api.default,
    { gasLimit, storageDepositLimit },
    ...args
  );

  if (result.isOk) {
    console.log(`Success ${queryFunction}`, output.toHuman());
  } else {
    console.error("Error", result.asErr);
    throw new Error(result.asErr);
  }
  return output.toHuman().Ok.Ok;
}

async function doHandshake(contractAddress, source, senderAddress, destinationAddress) {
  const { ContractPromise } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/api-contract@10.9.1/+esm"
  );
  const { api } = await initApi();
  const metadata = await loadContractMetadata();
  const contract = new ContractPromise(api, metadata, contractAddress);

  const metadataRpc = await api.rpc.state.getMetadata();
  api.registry.setMetadata(metadataRpc);
  
  const { BN, BN_ONE } = await import(
    "https://cdn.jsdelivr.net/npm/@polkadot/util@12.4.1/+esm"
  );

  const MAX_CALL_WEIGHT = new BN(11344007255).isub(BN_ONE);
  const PROOFSIZE = new BN(131072);

  const gasLimit = api?.registry.createType("WeightV2", {
    refTime: MAX_CALL_WEIGHT,
    proofSize: PROOFSIZE,
  });

  const storageDepositLimit = null;
  const extensionMod = await getPolkadotJsExtensionMod();
  const injector = await extensionMod.web3FromSource(source);

  const result = await contract.tx["handshake"]({
    gasLimit,
    storageDepositLimit,
  }, destinationAddress).signAndSend(senderAddress, { signer: injector.signer });
  console.log("handshake transaction result", result);
  return result.toHuman();
}

async function doAccountLookup(accountAddress) {
  const { SupportedChainId, resolveAddressToDomain } = await import(
    "https://cdn.jsdelivr.net/npm/@azns/resolver-core/+esm"
  );
  console.log(SupportedChainId)
  const { primaryDomain, error } = await resolveAddressToDomain(
    accountAddress,
    {
      chainId: SupportedChainId.AlephZero,
    },
  )
  // Print result
  if (error) console.log(error.message)
  else console.log("primary domain: ", primaryDomain)
  return primaryDomain;
}

async function fetchNumAccounts(contractAddress) {
  return await queryContract(contractAddress, "numAccounts");
}

async function fetchNumHandshakes(contractAddress) {
  return await queryContract(contractAddress, "numHandshakes");
}

fetchNumAccounts("5C8iyAnGiuWN2Dc4MZJMwDkw8U6CGYAJsDru5zFK5bUFof4Y").catch(
  console.error
);

fetchNumHandshakes("5C8iyAnGiuWN2Dc4MZJMwDkw8U6CGYAJsDru5zFK5bUFof4Y").catch(
  console.error
);

doAccountLookup("5H8rm9f9LE7VLqrL8qhmu4NAjwfPTrH8ShsyUqMBq6aDsaHb").catch(console.error);