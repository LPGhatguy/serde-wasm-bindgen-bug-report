import init, { get_value, give_value } from "../pkg";

async function main() {
	await init();

	const entity = get_value();
	console.log(entity);
	give_value(entity);
}

main();