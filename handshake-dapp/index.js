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
