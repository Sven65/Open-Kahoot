
export const replaceObjectById = (array: any[], idToReplace: string, newObject: Record<any, any>) => {
	const index = array.findIndex(obj => obj.id === idToReplace)
	if (index !== -1) {
		array[index] = { ...newObject, id: idToReplace } // Preserving the original ID
	}
}


export const deleteByKey = <T>(array: T[], prop: string, valueToDelete: any): T[] => {
	return array.filter(item => item[prop] !== valueToDelete)
}
