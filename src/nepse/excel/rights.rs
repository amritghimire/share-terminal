use crate::nepse::response::gain::{GainListResponse, RightListItem};
use xlsxwriter::*;

fn write_header(sheet: &mut Worksheet, text_wrap: &Format) -> Result<(), XlsxError> {
    let headers = vec![
        "Symbol",
        "Company Name",
        "Close Price",
        "Ratio",
        "Ratio (Frac)",
        "Units",
        "Sector",
        "Number of rights Record",
        "Book Closure Date",
        "Average rights",
        "Opening Date",
        "Closing Date",
        "Total Traded amount",
        "Total Traded Shares",
        "Total Trades",
        "Closing Price (date)",
    ];
    for (position, header) in headers.iter().enumerate() {
        sheet.write_string(0, position as u16, header, Some(&text_wrap))?;
    }
    Ok(())
}

fn format_to_f64(value: Option<String>) -> f64 {
    let empty: String = "0.0".to_string();
    value.unwrap_or(empty).parse().unwrap_or(0.0)
}

fn write_row(sheet: &mut Worksheet, row: u32, rights: &RightListItem) -> Result<(), XlsxError> {
    let empty: String = "".to_string();
    sheet.write_string(row, 0, rights.symbol.as_str(), None)?;
    sheet.write_string(
        row,
        1,
        rights.company_name.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;
    sheet.write_number(row, 2, rights.close_price.unwrap_or(0.0), None)?;

    sheet.write_string(row, 3, rights.ratio.as_str(), None)?;
    sheet.write_number(row, 4, rights.ratio_frac.unwrap_or(0.0), None)?;
    sheet.write_number(row, 5, format_to_f64(Some(rights.units.clone())), None)?;
    sheet.write_string(
        row,
        6,
        rights.sector_name.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;
    sheet.write_formula(
        row,
        7,
        format!("= COUNTIF(A:A, A{})", row + 1).as_str(),
        None,
    )?;
    sheet.write_string(
        row,
        8,
        rights.book_closure_date.as_ref().unwrap_or(&empty).as_str(),
        None,
    )?;

    sheet.write_formula(
        row,
        9,
        format!("= AVERAGEIF(A:A, A{}, E:E)", row + 1).as_str(),
        None,
    )?;
    sheet.write_string(row, 10, rights.opening_date.as_str(), None)?;
    sheet.write_string(row, 11, rights.closing_date.as_str(), None)?;
    sheet.write_string(
        row,
        12,
        rights
            .total_traded_amount
            .as_ref()
            .unwrap_or(&empty)
            .as_str(),
        None,
    )?;

    sheet.write_string(
        row,
        13,
        rights
            .total_traded_shares
            .as_ref()
            .unwrap_or(&empty)
            .as_str(),
        None,
    )?;
    sheet.write_number(row, 14, rights.total_trades.unwrap_or(0.0), None)?;
    sheet.write_string(
        row,
        15,
        rights
            .closing_date_price
            .as_ref()
            .unwrap_or(&empty)
            .as_str(),
        None,
    )?;

    Ok(())
}

pub fn write_to_excel(
    file_path: String,
    response: Result<GainListResponse<RightListItem>, reqwest::Error>,
) -> Result<(), XlsxError> {
    if let Ok(list_response) = response {
        let workbook = Workbook::new(file_path.as_str());
        let text_wrap = workbook.add_format().set_text_wrap();
        let mut sheet = workbook.add_worksheet(Some("rights"))?;
        write_header(&mut sheet, &text_wrap)?;

        for (row, rights) in list_response.data.iter().enumerate() {
            write_row(&mut sheet, row as u32 + 1, rights)?;
        }
        sheet.set_column(1, 1, 30.0, None)?;
        sheet.set_column(2, 14, 10.0, Some(&text_wrap))?;
        sheet.autofilter(0, 0, list_response.data.len() as u32 + 2, 15)?;

        println!("Press Ctrl+Shift+F9 in case of libra office to recalculate formulas.");
        workbook.close()?;
    } else {
        println!("Error occured: {:?}", response.unwrap_err());
    }
    Ok(())
}
