export function getFormStringValue(
  formData: FormData,
  fieldName: string,
): string {
  const value = formData.get(fieldName);

  if (typeof value !== "string" || value.trim() === "") {
    throw new Error(`Missing ${fieldName}`);
  }

  return value.trim();
}

export function getFormNumberValue(
  formData: FormData,
  fieldName: string,
): number {
  const value = getFormStringValue(formData, fieldName);
  const parsedValue = Number(value);

  if (Number.isNaN(parsedValue)) {
    throw new Error(`Invalid ${fieldName}`);
  }

  return parsedValue;
}
